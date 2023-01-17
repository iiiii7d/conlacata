use std::{
    fmt::Display,
    fs,
    path::{Path, PathBuf},
};

use clap::Parser;
use color_eyre::{eyre::eyre, Report, Result};
use either::Either;
use indexmap::IndexMap;
use owo_colors::OwoColorize;
use regex::Regex;
use serde::{Deserialize, Serialize};

use crate::{
    modules::{
        ipa::conlang_to_ipa,
        orthography::Orthography,
        parts_of_speech::{PartOfSpeech, PartsOfSpeech},
    },
    types::{ConlangString, IpaString},
    CliOptions,
};

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct Word<'a> {
    pub spelling: ConlangString,
    pub pos: String,
    pub definitions: Vec<String>,
    #[serde(default = "Vec::new")]
    pub categories: Vec<String>,
    #[serde(skip)]
    pub _psos: Option<&'a PartsOfSpeech>,
}
impl Display for Word<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{} {}\n{}{}",
            self.spelling.yellow().bold(),
            format!(
                "({})",
                if let Some(pos) = self.get_pos() {
                    pos.abbrev.to_owned()
                } else {
                    self.pos.to_owned()
                }
            )
            .green(),
            self.definitions.join(", "),
            format!("\nCategories: {}", self.categories.join(", ")).bright_black()
        )
    }
}
impl<'a> Word<'a> {
    pub fn get_pos(&self) -> Option<&'a PartOfSpeech> {
        self._psos?.parts.iter().find(|&pos| pos.name == self.pos)
    }
    pub fn ipa(&self, ortho: &Orthography) -> IpaString {
        conlang_to_ipa(self.spelling.to_owned(), ortho)
    }

    #[allow(clippy::unwrap_in_result)] // TODO fix
    pub fn conjugate(
        &self,
        conjugations: IndexMap<String, Either<bool, String>>,
        ortho: &Orthography,
    ) -> Result<(ConlangString, IpaString)> {
        let mut spelling: ConlangString = self.spelling.to_owned();
        let mut ipa: IpaString = self.ipa(ortho);
        let psos = self
            ._psos
            .ok_or_else(|| eyre!("No parts of speech loaded"))?;
        for (conj_name, dim_name) in conjugations {
            let conj = self
                .get_pos()
                .ok_or_else(|| {
                    eyre!(
                        "No part of speech named {}\n{}",
                        self.pos,
                        format!(
                            "Valid parts of speech: {}",
                            psos.parts
                                .iter()
                                .map(|p| p.name.to_owned())
                                .collect::<Vec<_>>()
                                .join(", ")
                        )
                        .yellow()
                    )
                })?
                .conjugations
                .iter()
                .find(|c| c.name == conj_name)
                .ok_or_else(|| {
                    eyre!(
                        "No conjugation named {}\n{}",
                        conj_name,
                        format!(
                            "Valid conjugations: {}",
                            self.get_pos()
                                .unwrap()
                                .conjugations
                                .iter()
                                .map(|c| c.name.to_string())
                                .collect::<Vec<_>>()
                                .join("\n")
                        )
                        .yellow()
                    )
                })?;
            let dim = if let Either::Right(dim_name) = dim_name {
                conj.dimensions
                    .iter()
                    .find(|d| d.name == dim_name)
                    .ok_or_else(|| {
                        eyre!(
                            "No dimension named {}\n{}",
                            dim_name,
                            format!(
                                "Valid dimensions: {}",
                                conj.dimensions
                                    .iter()
                                    .map(|d| d.name.to_string())
                                    .collect::<Vec<_>>()
                                    .join("\n")
                            )
                            .yellow()
                        )
                    })?
            } else if let Either::Left(changed) = dim_name {
                conj.dimensions
                    .iter()
                    .find(|d| d.original_form != changed)
                    .ok_or_else(|| eyre!("No default dimension"))?
            } else {
                unreachable!()
            };
            for rule in &dim.rules {
                let spelling_regex = Regex::new(&rule.spelling_regex.to_owned())?;
                let ipa_regex = Regex::new(
                    &rule
                        .ipa_regex
                        .to_owned()
                        .unwrap_or_else(|| rule.spelling_regex.to_owned()),
                )?;
                let spelling_subst = rule.spelling_subst.to_owned();
                let ipa_subst = rule
                    .ipa_subst
                    .to_owned()
                    .unwrap_or_else(|| conlang_to_ipa(rule.spelling_subst.to_owned(), ortho));
                if spelling_regex.is_match(&spelling) {
                    spelling = spelling_regex
                        .replace_all(&spelling, spelling_subst.as_str())
                        .into();
                    ipa = ipa_regex.replace_all(&ipa, ipa_subst.as_str()).into();
                }
            }
        }
        Ok((spelling, ipa))
    }
}

#[derive(Serialize, Deserialize, Default)]
pub struct Lexicon<'a> {
    pub words: Vec<Word<'a>>,
}
impl Display for Lexicon<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.words
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<_>>()
                .join("\n\n")
        )
    }
}
impl<'a> Lexicon<'a> {
    pub fn from_folder(path: PathBuf, psos: &'a PartsOfSpeech) -> Result<Self> {
        let mut lexicons: Vec<Lexicon> = vec![];
        for path in fs::read_dir(path)? {
            let path = path?.path();
            let mut file_data: Lexicon = toml::from_str(&fs::read_to_string(&path)?)?;
            file_data.words.iter_mut().try_for_each(|w| {
                w._psos = Some(psos);
                if let Some(stem) = path.file_stem() {
                    w.categories.push(
                        stem.to_str()
                            .ok_or_else(|| eyre!("Invalid unicode"))?
                            .to_owned(),
                    );
                }
                Result::<_, Report>::Ok(())
            })?;
            lexicons.push(file_data);
        }
        Ok(lexicons
            .into_iter()
            .reduce(|mut a, b| {
                a.words.extend(b.words);
                a
            })
            .unwrap_or_default())
    }
    pub fn from_lang_folder(lang_folder: &Path, psos: &'a PartsOfSpeech) -> Result<Self> {
        let file = lang_folder.join("lexicon");
        Lexicon::from_folder(file, psos)
    }
}

#[derive(Parser)]
pub struct LexiconOptions {
    search: Option<String>,
}
impl CliOptions for LexiconOptions {
    fn run(&self, lang_folder: PathBuf) -> Result<()> {
        let psos = PartsOfSpeech::from_lang_folder(&lang_folder)?;
        let mut lexicon = Lexicon::from_lang_folder(&lang_folder, &psos)?;
        if let Some(query) = &self.search {
            let query = Regex::new(query)?;
            lexicon.words.retain(|w| query.is_match(&w.spelling));
        }
        println!("{lexicon}");
        Ok(())
    }
}

const CUSTOM_WORD_ERROR: &str =
    "Use `<part_of_speech>:<custom_word>` for a word that is not in the lexicon.";
const CONJ_ERROR: &str = "Use `<conjugation>` (non-dimensional) / `<conjugation>:<dimension>` (dimensional) for specifying conjugations.";

#[derive(Parser)]
pub struct ConjugationOptions {
    word: ConlangString,
    conjugations: Vec<String>,
}
impl CliOptions for ConjugationOptions {
    fn run(&self, lang_folder: PathBuf) -> Result<()> {
        let psos = PartsOfSpeech::from_lang_folder(&lang_folder)?;
        let ortho = Orthography::from_lang_folder(&lang_folder)?;
        let lexicon = Lexicon::from_lang_folder(&lang_folder, &psos)?;
        let word = if let Some(word) = lexicon.words.iter().find(|w| w.spelling == self.word) {
            word.to_owned()
        } else {
            Word {
                spelling: self
                    .word
                    .split(':')
                    .nth(1)
                    .ok_or_else(|| eyre!("{CUSTOM_WORD_ERROR}"))?
                    .into(),
                pos: self
                    .word
                    .split(':')
                    .next()
                    .ok_or_else(|| eyre!("{CUSTOM_WORD_ERROR}"))?
                    .to_owned(),
                _psos: Some(&psos),
                ..Default::default()
            }
        };
        if !psos.parts.iter().any(|p| p.name == word.pos) {
            return Err(eyre!(
                "No part of speech named {}\n{}",
                word.pos,
                format!(
                    "Valid parts of speech: {}",
                    psos.parts
                        .iter()
                        .map(|p| p.name.to_owned())
                        .collect::<Vec<_>>()
                        .join(", ")
                )
                .yellow()
            ));
        }

        let conjugations = self
            .conjugations
            .iter()
            .map(|c| {
                let conj_name = c
                    .split(':')
                    .next()
                    .ok_or_else(|| eyre!("{CONJ_ERROR}"))?
                    .to_owned();
                let dim_name = if let Some(dim) = c.split(':').nth(1) {
                    Either::Right(dim.to_owned())
                } else {
                    Either::Left(true)
                };
                Ok((conj_name, dim_name))
            })
            .collect::<Result<IndexMap<_, _>>>()?;
        let (spelling, ipa) = word.conjugate(conjugations, &ortho)?;
        println!("{}\n{}", spelling, ipa.bright_black());
        Ok(())
    }
}
