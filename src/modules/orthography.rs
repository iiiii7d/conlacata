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

#[derive(Serialize, Deserialize)]
pub struct Letter {
    pub forms: Vec<ConlangString>,
    pub pronunciation: IpaString,
}
impl Display for Letter {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}\n{}",
            self.forms
                .iter()
                .map(|s| s.yellow().bold().to_string())
                .collect::<Vec<_>>()
                .join(" / "),
            format!("[{}]", self.pronunciation).italic()
        )
    }
}
#[derive(Serialize, Deserialize)]
pub struct Orthography {
    #[serde(default = "Vec::new")]
    pub letters: Vec<Letter>,
    #[serde(default = "Vec::new")]
    pub other_chars: Vec<Letter>,
}
impl Display for Orthography {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.letters
                .iter()
                .map(|l| l
                    .to_string()
                    .split('\n')
                    .map(|line| format!("\t{line}"))
                    .collect::<Vec<_>>()
                    .join("\n"))
                .enumerate()
                .map(|(i, e)| format!("{}: {}", i + 1, e))
                .collect::<Vec<_>>()
                .join("\n")
        )
    }
}
impl Orthography {
    pub fn from_file(path: PathBuf) -> Result<Self> {
        let content = std::fs::read_to_string(path)?;
        Ok(toml::from_str(&content)?)
    }
    pub fn from_lang_folder(lang_folder: &Path) -> Result<Self> {
        let file = lang_folder.join("orthography.toml");
        Self::from_file(file)
    }
}

#[derive(Parser)]
pub struct OrthographyOptions;
impl CliOptions for OrthographyOptions {
    fn run(&self, lang_folder: PathBuf) -> Result<()> {
        let data = Orthography::from_lang_folder(&lang_folder)?;
        println!("{data}");
        Ok(())
    }
}
