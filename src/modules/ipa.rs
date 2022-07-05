use std::path::PathBuf;
use ansi_term::Color::White;
use clap::Parser;
use unicode_segmentation::UnicodeSegmentation;
use crate::modules::orthography::Orthography;
use crate::types::{ConlangString, IpaString};

pub fn conlang_to_ipa(input: ConlangString, ortho: Orthography) -> IpaString {
    let mut ipa = String::new();
    let mut graphemes = input.graphemes(true).enumerate();
    'outer: while let Some((index, grapheme)) = graphemes.next() {
        if grapheme.trim().is_empty() {
            ipa += grapheme;
            continue;
        }
        for letter in ortho.other_chars.iter().chain(ortho.letters.iter()) {
            for form in letter.forms.iter() {
                if input.graphemes(true).skip(index)
                    .collect::<String>().starts_with(form) {
                    ipa += &*letter.pronunciation;
                    for _ in 0..(form.graphemes(true).count() - 1) {
                        graphemes.next();
                    }
                    continue 'outer;
                }
            }
        }
        ipa += grapheme
    }
    ipa
}

#[derive(Parser)]
pub struct IpaOptions {
    lang_folder: PathBuf,
    /// Input in conlang to translate to IPA
    input: ConlangString,
}
impl IpaOptions {
    pub fn run(&self) {
        let file = self.lang_folder.join("orthography.toml");
        let ortho = Orthography::from_file(file);
        println!("{}\n[{}]",
            White.dimmed().paint(self.input.to_owned()),
            conlang_to_ipa(self.input.to_owned(), ortho));
    }
}