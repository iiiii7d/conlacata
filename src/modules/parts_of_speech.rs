use std::{
    fmt::Display,
    path::{Path, PathBuf},
};

use clap::Parser;
use color_eyre::Result;
use owo_colors::OwoColorize;
use serde::{Deserialize, Serialize};

use crate::{
    types::{ConlangString, IpaString},
    CliOptions,
};

const RETURNS_FALSE: fn() -> bool = || false;

#[derive(Serialize, Deserialize)]
pub struct Rule {
    pub spelling_regex: ConlangString,
    pub spelling_subst: ConlangString,
    pub ipa_regex: Option<IpaString>,
    pub ipa_subst: Option<IpaString>,
}
impl Display for Rule {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{} \u{2192} {}{}",
            self.spelling_regex,
            self.spelling_subst,
            if let (Some(ipa_regex), Some(ipa_subst)) =
                (self.ipa_regex.to_owned(), self.ipa_subst.to_owned())
            {
                format!(" ({ipa_regex} \u{2192} {ipa_subst})")
                    .bright_black()
                    .to_string()
            } else {
                "".into()
            }
        )
    }
}

#[derive(Serialize, Deserialize)]
pub struct Dimension {
    pub name: String,
    #[serde(default = "String::new")]
    pub description: String,
    #[serde(default = "RETURNS_FALSE")]
    pub original_form: bool,
    #[serde(default = "Vec::new")]
    pub rules: Vec<Rule>,
}
impl Display for Dimension {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}{} {}{}{}",
            self.name.yellow().bold(),
            if self.original_form {
                " (original)".green().to_string()
            } else {
                "".into()
            },
            self.description.bright_black(),
            if self.rules.is_empty() { "" } else { "\n" },
            self.rules
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<_>>()
                .join("\n")
        )
    }
}

#[derive(Serialize, Deserialize)]
pub struct Conjugation {
    pub name: String,
    #[serde(default = "String::new")]
    pub description: String,
    #[serde(default = "RETURNS_FALSE")]
    pub multi_dimensional: bool,
    #[serde(default = "Vec::new")]
    pub dimensions: Vec<Dimension>,
}
impl Display for Conjugation {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}{} {}\n{}",
            self.name.yellow().bold(),
            if self.multi_dimensional {
                " (multi-dimensional)".green().to_string()
            } else {
                "".into()
            },
            self.description.bright_black(),
            self.dimensions
                .iter()
                .map(|d| d
                    .to_string()
                    .split('\n')
                    .map(|line| format!("\t{line}"))
                    .collect::<Vec<_>>()
                    .join("\n"))
                .collect::<Vec<_>>()
                .join("\n")
        )
    }
}

#[derive(Serialize, Deserialize)]
pub struct PartOfSpeech {
    pub name: String,
    #[serde(default = "String::new")]
    pub description: String,
    pub abbrev: String,
    #[serde(default = "Vec::new")]
    pub conjugations: Vec<Conjugation>,
}
impl Display for PartOfSpeech {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{} ({}) {}\n{}",
            self.name.yellow().bold(),
            self.abbrev.to_owned(),
            self.description.bright_black(),
            self.conjugations
                .iter()
                .map(|c| c
                    .to_string()
                    .split('\n')
                    .map(|line| format!("\t{line}"))
                    .collect::<Vec<_>>()
                    .join("\n"))
                .collect::<Vec<_>>()
                .join("\n")
        )
    }
}

#[derive(Serialize, Deserialize, Default)]
pub struct PartsOfSpeech {
    pub parts: Vec<PartOfSpeech>,
}
impl Display for PartsOfSpeech {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.parts
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<_>>()
                .join("\n")
        )
    }
}
impl PartsOfSpeech {
    pub fn from_file(path: PathBuf) -> Result<Self> {
        let content = std::fs::read_to_string(path)?;
        Ok(toml::from_str(&content)?)
    }
    pub fn from_lang_folder(lang_folder: &Path) -> Result<Self> {
        let file = lang_folder.join("parts_of_speech.toml");
        Self::from_file(file)
    }
}

#[derive(Parser)]
pub struct PartsOfSpeechOptions;
impl CliOptions for PartsOfSpeechOptions {
    fn run(&self, lang_folder: PathBuf) -> Result<()> {
        let data = PartsOfSpeech::from_lang_folder(&lang_folder)?;
        println!("{data}");
        Ok(())
    }
}
