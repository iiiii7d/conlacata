use std::collections::HashMap;
use std::fmt::Display;
use std::fs;
use std::path::PathBuf;
use ansi_term::Color::{Green, White, Yellow};
use serde::{Deserialize, Serialize};
use clap::Parser;
use either::Either;
use crate::modules::ipa::conlang_to_ipa;
use crate::modules::orthography::Orthography;
use crate::modules::parts_of_speech::{Conjugation, Dimension, PartOfSpeech, PartsOfSpeech};

#[derive(Serialize, Deserialize, Default)]
pub struct Word<'a> {
    pub spelling: String,
    pub pos: String,
    pub definitions: Vec<String>,
    #[serde(default = "Vec::new")]
    pub categories: Vec<String>,
    #[serde(skip)]
    pub _psos: Option<&'a PartsOfSpeech>,
}
impl Display for Word<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} {}\n{}\n{}",
            Yellow.bold().paint(self.spelling.to_owned()),
            Green.paint(format!("({})", if let Some(pos) = self.get_pos() {
                pos.abbrev.to_owned()
            } else {
                self.pos.to_owned()
            })),
            self.definitions.join(", "),
            White.dimmed().paint(format!(
                "\nCategories: {}", self.categories.join(", "))))
    }
}
impl<'a> Word<'a> {
    pub fn get_pos(&self) -> Option<&'a PartOfSpeech> {
        for pos in &self._psos?.parts {
            if pos.name == self.pos {
                return Some(pos);
            }
        }
        None
    }
    pub fn ipa(&self, ortho: Orthography) -> String {
        conlang_to_ipa(self.spelling.to_owned(), ortho)
    }

    pub fn conjugate(&self, conjugations: HashMap<String, Either<bool, String>>) -> Result<String, String> {
        let mut spelling = self.spelling.to_owned();
        self._psos.ok_or("No parts of speech loaded")?;
        for (conj_name, dim_name) in conjugations {
            let conj = self.get_pos().unwrap().conjugations.iter()
                .find(|c| c.name == conj_name)
                .ok_or(format!("No conjugation named {}", conj_name))?;
            let dim = if let Either::Right(dim_name) = dim_name {
                self._psos.unwrap().dimensions.iter()
                    .find(|d| d.name == dim_name)
                    .ok_or(format!("No dimension named {}", dim_name))?
            } else {
                conj.dimensions.iter()
                    .find(|d| d.name == "default")
                    .ok_or("No default dimension")?
            };
        }
        "".into()
    }
}

#[derive(Serialize, Deserialize, Default)]
pub struct Lexicon<'a> {
    pub words: Vec<Word<'a>>
}
impl Display for Lexicon<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.words.iter()
            .map(|w| w.to_string())
            .collect::<Vec<_>>()
            .join("\n\n"))
    }
}
impl<'a> Lexicon<'a> {
    pub fn from_folder(path: PathBuf, psos: &'a PartsOfSpeech) -> Self {
        let mut lexicons: Vec<Lexicon> = vec![];
        for path in fs::read_dir(path).unwrap() {
            let path = path.unwrap().path();
            let mut file_data: Lexicon = toml::from_str(&fs::read_to_string(&path)
                .unwrap()).unwrap();
            file_data.words.iter_mut()
                .for_each(|w| {
                    w._psos = Some(psos);
                    w.categories.push(path.file_stem().unwrap()
                        .to_str().unwrap().to_string());
                });
            lexicons.push(file_data);
        }
        lexicons.into_iter().reduce(|mut a, b| {
            a.words.extend(b.words);
            a
        }).unwrap_or_default()
    }
}

#[derive(Parser)]
pub struct LexiconOptions {
    lang_folder: PathBuf,
    search: Option<String>
}
impl LexiconOptions {
    pub fn run(&self) {
        let file = self.lang_folder.join("lexicon/");
        let psos = PartsOfSpeech::from_file(
            self.lang_folder.join("parts_of_speech.toml"));
        let mut lexicon = Lexicon::from_folder(file, &psos);
        if let Some(query) = &self.search {
            lexicon.words = lexicon.words.into_iter()
                .filter(|w| w.spelling.contains(query))
                .collect(); // TODO Regex thingy
        }
        println!("{}", lexicon);
    }
}