use std::fmt::Display;
use std::path::PathBuf;
use ansi_term::Color::Yellow;
use ansi_term::Style;
use serde::{Serialize, Deserialize};
use clap::Parser;
use crate::types::{ConlangString, IpaString};

#[derive(Serialize, Deserialize)]
pub struct Letter {
    pub forms: Vec<ConlangString>,
    pub pronunciation: IpaString,
}
impl Display for Letter {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}\n{}",
        self.forms.iter()
            .map(|s| Yellow.bold().paint(s).to_string())
            .collect::<Vec<_>>()
            .join(" / "),
        Style::new().italic().paint(format!("[{}]", self.pronunciation)))
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
        write!(f, "{}",
        self.letters.iter()
            .map(|l| l.to_string()
                .split('\n')
                .map(|line| format!("\t{}", line))
                .collect::<Vec<_>>()
                .join("\n"))
            .enumerate()
            .map(|(i, e)| format!("{}: {}", i+1, e))
            .collect::<Vec<_>>()
            .join("\n"))
    }
}
impl Orthography {
    pub fn from_file(path: PathBuf) -> Self {
        let content = std::fs::read_to_string(path).unwrap();
        toml::from_str(&content).unwrap()
    }
}

#[derive(Parser)]
pub struct OrthographyOptions {
    lang_folder: PathBuf
}
impl OrthographyOptions {
    pub fn run(&self) {
        let file = self.lang_folder.join("orthography.toml");
        let data = Orthography::from_file(file);
        println!("{}", data);
    }
}