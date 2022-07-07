use std::fmt::Display;
use std::fs;
use std::path::PathBuf;
use ansi_term::Color::{White, Yellow};
use serde::{Deserialize, Serialize};
use clap::Parser;
use crate::modules::parts_of_speech::{PartOfSpeech, PartsOfSpeech};

#[derive(Serialize, Deserialize)]
pub struct Word {
    pub spelling: String,
    pub pos: String,
    pub definitions: Vec<String>,
    pub categories: Vec<String>
}
impl Display for Word {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} ({})\n{}\nCategories: {}",
            Yellow.bold().paint(self.spelling.to_owned()),
            White.dimmed().paint(self.pos.to_owned()),
            self.definitions.join(", "),
            White.dimmed().paint(self.categories.join(", ")))
    }
}
impl Word {
    pub fn get_pos<'a>(&self, pos: &'a PartsOfSpeech) -> Option<&'a PartOfSpeech> {
        for pos in &pos.parts {
            if pos.name == self.pos {
                return Some(pos);
            }
        }
        None
    }
}

#[derive(Serialize, Deserialize)]
pub struct Lexicon {
    pub words: Vec<Word>
}
impl Display for Lexicon {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.words.iter()
            .map(|w| w.to_string())
            .collect::<Vec<_>>()
            .join("\n\n"))
    }
}
impl Lexicon {
    pub fn from_folder(path: PathBuf) -> Self {
        let mut lexicons: Vec<Lexicon> = vec![];
        for path in fs::read_dir(path).unwrap() {
            let path = path.unwrap().path();
            lexicons.push(toml::from_str(&fs::read_to_string(path)
                .unwrap()).unwrap());
        }
        lexicons.into_iter().reduce(|mut a, b| {
            a.words.extend(b.words);
            a
        }).unwrap_or(Lexicon { words: vec![] })
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
        let lexicon = Lexicon::from_folder(file);
        let pos = PartsOfSpeech::from_file(
            self.lang_folder.join("parts_of_speech.toml"));
        println!("{}", lexicon);
    }
}