use std::error::Error;
use std::fmt::Display;
use std::fs;
use std::path::PathBuf;
use ansi_term::Color::{Green, White, Yellow};
use serde::{Deserialize, Serialize};
use clap::Parser;
use either::Either;
use indexmap::IndexMap;
use regex::Regex;
use crate::modules::ipa::conlang_to_ipa;
use crate::modules::orthography::Orthography;
use crate::modules::parts_of_speech::{PartOfSpeech, PartsOfSpeech};
use crate::types::{ConlangString, IpaString, ResultAnyError};

#[derive(Serialize, Deserialize, Default, Clone)]
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
        write!(f, "{} {}\n{}{}",
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
    pub fn ipa(&self, ortho: &Orthography) -> String {
        conlang_to_ipa(self.spelling.to_owned(), ortho)
    }

    pub fn conjugate(&self, conjugations: IndexMap<String, Either<bool, String>>,
                     ortho: &Orthography) -> ResultAnyError<(ConlangString, IpaString)> {
        let mut spelling: ConlangString = self.spelling.to_owned();
        let mut ipa: IpaString = self.ipa(ortho);
        self._psos.ok_or("No parts of speech loaded")?;
        for (conj_name, dim_name) in conjugations {
            let conj = self.get_pos().ok_or(
                format!("No part of speech named {}\n{}", self.pos,
                    Yellow.paint(format!("Valid parts of speech: {}",
                        self._psos.unwrap().parts.iter()
                            .map(|p| p.name.to_owned())
                            .collect::<Vec<_>>()
                            .join(", "))))
            )?.conjugations.iter()
                .find(|c| c.name == conj_name)
                .ok_or(format!("No conjugation named {}\n{}", conj_name,
                    Yellow.paint(format!("Valid conjugations: {}",
                    self.get_pos().unwrap().conjugations.iter()
                        .map(|c| c.name.to_string())
                        .collect::<Vec<_>>()
                        .join("\n")))
                ))?;
            let dim = if let Either::Right(dim_name) = dim_name {
                conj.dimensions.iter()
                    .find(|d| d.name == dim_name)
                    .ok_or(format!("No dimension named {}\n{}", dim_name,
                        Yellow.paint(format!("Valid dimensions: {}",
                        conj.dimensions.iter()
                        .map(|d| d.name.to_string())
                        .collect::<Vec<_>>()
                        .join("\n")))
                    ))?
            } else if let Either::Left(changed) = dim_name {
                conj.dimensions.iter()
                    .find(|d| d.original_form != changed)
                    .ok_or("No default dimension")?
            } else {unreachable!()};
            for rule in dim.rules.iter() {
                let spelling_regex = Regex::new(&*rule.spelling_regex.to_owned())?;
                let ipa_regex = Regex::new(&*rule.ipa_regex.to_owned().unwrap_or_else(
                    || rule.spelling_regex.to_owned()))?;
                let spelling_subst = rule.spelling_subst.to_owned();
                let ipa_subst = rule.ipa_subst.to_owned().unwrap_or_else(
                    || conlang_to_ipa(rule.spelling_subst.to_owned(), ortho));
                if spelling_regex.is_match(&spelling) {
                    spelling = spelling_regex.replace_all(&spelling, spelling_subst).to_string();
                    ipa = ipa_regex.replace_all(&ipa, ipa_subst).to_string();
                }
            }
        }
        Ok((spelling, ipa))
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
    pub fn from_folder(path: PathBuf, psos: &'a PartsOfSpeech) -> ResultAnyError<Self> {
        let mut lexicons: Vec<Lexicon> = vec![];
        for path in fs::read_dir(path)? {
            let path = path?.path();
            let mut file_data: Lexicon = toml::from_str(&fs::read_to_string(&path)
                ?)?;
            file_data.words.iter_mut()
                .try_for_each(|w| {
                    w._psos = Some(psos);
                    if let Some(stem) = path.file_stem() {
                        w.categories.push(stem.to_str()
                            .ok_or("Invalid unicode")?.to_owned());
                    }
                    Result::<(), Box<dyn Error>>::Ok(())
                })?;
            lexicons.push(file_data);
        }
        Ok(lexicons.into_iter().reduce(|mut a, b| {
            a.words.extend(b.words);
            a
        }).unwrap_or_default())
    }
    pub fn from_lang_folder(lang_folder: PathBuf, psos: &'a PartsOfSpeech) -> ResultAnyError<Self> {
        let file = lang_folder.join("lexicon");
        Lexicon::from_folder(file, psos)
    }
}

#[derive(Parser)]
pub struct LexiconOptions {
    lang_folder: PathBuf,
    search: Option<String>
}
impl LexiconOptions {
    pub fn run(&self) -> ResultAnyError<()> {
        let file = self.lang_folder.join("lexicon/");
        let psos = PartsOfSpeech::from_file(
            self.lang_folder.join("parts_of_speech.toml"))?;
        let mut lexicon = Lexicon::from_folder(file, &psos)?;
        if let Some(query) = &self.search {
            let query = Regex::new(&*query)?;
            lexicon.words = lexicon.words.into_iter()
                .filter(|w| query.is_match(&w.spelling))
                .collect();
        }
        println!("{}", lexicon);
        Ok(())
    }
}

const CUSTOM_WORD_ERROR: &str = "Use `<part_of_speech>:<custom_word>` for a word that is not in the lexicon.";
const CONJ_ERROR: &str = "Use `<conjugation>` (non-dimensional) / `<conjugation>:<dimension>` (dimensional) for specifying conjugations.";

#[derive(Parser)]
pub struct ConjugationOptions {
    lang_folder: PathBuf,
    word: ConlangString,
    conjugations: Vec<String>
}
impl ConjugationOptions {
    pub fn run(&self) -> ResultAnyError<()> {
        let psos = PartsOfSpeech::from_lang_folder(self.lang_folder.to_owned())?;
        let ortho = Orthography::from_lang_folder(self.lang_folder.to_owned())?;
        let lexicon = Lexicon::from_lang_folder(self.lang_folder.to_owned(), &psos)?;
        let word = if let Some(word) = lexicon.words.iter()
            .find(|w| w.spelling == self.word) {word.to_owned()} else {
            Word {
                spelling: self.word.split(':').nth(1)
                    .ok_or(CUSTOM_WORD_ERROR)?.to_string(),
                pos: self.word.split(':').next()
                    .ok_or(CUSTOM_WORD_ERROR)?.to_string(),
                _psos: Some(&psos),
                ..Default::default()
            }
        };

        let conjugations = self.conjugations.iter().map(|c| {
            let conj_name = c.split(':').next().ok_or(CONJ_ERROR)?.to_string();
            let dim_name = if let Some(dim) = c.split(':').nth(1)
            {Either::Right(dim.to_string())} else {Either::Left(true)};
            Ok((conj_name, dim_name))
        }).collect::<ResultAnyError<IndexMap<_, _>>>()?;
        let (spelling, ipa) = word.conjugate(conjugations, &ortho)?;
        println!("{}\n{}", spelling, White.dimmed().paint(ipa));
        Ok(())
    }
}