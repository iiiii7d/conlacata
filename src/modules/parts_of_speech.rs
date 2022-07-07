use std::fmt::Display;
use std::path::PathBuf;
use ansi_term::Color::{Green, White, Yellow};
use serde::{Deserialize, Serialize};
use clap::Parser;
use crate::types::{ConlangString, IpaString, ResultAnyError};

const RETURNS_FALSE: fn() -> bool = || false;

#[derive(Serialize, Deserialize)]
pub struct Rule {
    pub spelling_regex: ConlangString,
    pub spelling_subst: ConlangString,
    pub ipa_regex: Option<IpaString>,
    pub ipa_subst: Option<IpaString>
}
impl Display for Rule {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} → {}{}", self.spelling_regex, self.spelling_subst,
            if let (Some(ipa_regex), Some(ipa_subst))
                = (self.ipa_regex.to_owned(), self.ipa_subst.to_owned()) {
                White.dimmed().paint(format!(" ({} → {})", ipa_regex, ipa_subst))
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
    pub rules: Vec<Rule>
}
impl Display for Dimension {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}{} {}{}{}",
            Yellow.bold().paint(self.name.to_owned()),
            if self.original_form {
                Green.paint(" (original)")
            } else {
                "".into()
            },
            White.dimmed().paint(self.description.to_owned()),
            if !self.rules.is_empty() {"\n"} else {""},
            self.rules.iter()
                   .map(|r| r.to_string())
                   .collect::<Vec<_>>()
                   .join("\n"))
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
        write!(f, "{}{} {}\n{}",
            Yellow.bold().paint(self.name.to_owned()),
            if self.multi_dimensional {
                Green.paint(" (multi-dimensional)")
            } else {
                "".into()
            },
            White.dimmed().paint(self.description.to_owned()),
            self.dimensions.iter()
                   .map(|d| d.to_string()
                       .split('\n')
                       .map(|line| format!("\t{}", line))
                       .collect::<Vec<_>>()
                       .join("\n"))
                   .collect::<Vec<_>>()
                   .join("\n"))
    }
}

#[derive(Serialize, Deserialize)]
pub struct PartOfSpeech {
    pub name: String,
    #[serde(default = "String::new")]
    pub description: String,
    pub abbrev: String,
    #[serde(default = "Vec::new")]
    pub conjugations: Vec<Conjugation>
}
impl Display for PartOfSpeech {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} ({}) {}\n{}",
            Yellow.bold().paint(self.name.to_owned()),
            self.abbrev.to_owned(),
            White.dimmed().paint(self.description.to_owned()),
            self.conjugations.iter()
                .map(|c| c.to_string()
                    .split('\n')
                    .map(|line| format!("\t{}", line))
                    .collect::<Vec<_>>()
                    .join("\n"))
                .collect::<Vec<_>>()
                .join("\n"))
    }
}

#[derive(Serialize, Deserialize, Default)]
pub struct PartsOfSpeech {
    pub parts: Vec<PartOfSpeech>
}
impl Display for PartsOfSpeech {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.parts.iter()
            .map(|p| p.to_string())
            .collect::<Vec<_>>()
            .join("\n"))
    }
}
impl PartsOfSpeech {
    pub fn from_file(path: PathBuf) -> ResultAnyError<Self> {
        let content = std::fs::read_to_string(path)?;
        Ok(toml::from_str(&content)?)
    }
    pub fn from_lang_folder(lang_folder: PathBuf) -> ResultAnyError<Self> {
        let file = lang_folder.join("parts_of_speech.toml");
        PartsOfSpeech::from_file(file)
    }
}

#[derive(Parser)]
pub struct PartsOfSpeechOptions {
    lang_folder: PathBuf
}
impl PartsOfSpeechOptions {
    pub fn run(&self) -> ResultAnyError<()> {
        let data = PartsOfSpeech::from_lang_folder(self.lang_folder.to_owned())?;
        println!("{}", data);
        Ok(())
    }
}