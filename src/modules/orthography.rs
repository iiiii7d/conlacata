use std::fmt::Display;
use std::path::PathBuf;
use ansi_term::Color::Yellow;
use ansi_term::Style;
use serde::{Serialize, Deserialize};
use clap::Parser;

#[derive(Serialize, Deserialize)]
pub struct Letter {
    forms: Vec<String>,
    pronunciation: String,
}
impl Display for Letter {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}\n{}",
        self.forms.iter()
            .map(|s| Yellow.bold().paint(s).to_string())
            .collect::<Vec<_>>()
            .join(" / "),
        Style::new().italic().paint(self.pronunciation.to_owned()))
    }
}
#[derive(Serialize, Deserialize)]
pub struct Orthography {
    letters: Vec<Letter>,
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