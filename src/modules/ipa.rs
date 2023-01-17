use std::path::PathBuf;

use ansi_term::Color::White;
use clap::Parser;
use unicode_segmentation::UnicodeSegmentation;

use crate::{
    modules::orthography::Orthography,
    types::{ConlangString, IpaString, ResultAnyError},
    CliOptions,
};

pub fn conlang_to_ipa(input: ConlangString, ortho: &Orthography) -> IpaString {
    let mut ipa = String::new();
    let mut graphemes = input.graphemes(true).enumerate();
    'outer: while let Some((index, grapheme)) = graphemes.next() {
        if grapheme.trim().is_empty() {
            ipa += grapheme;
            continue;
        }
        for letter in ortho.other_chars.iter().chain(ortho.letters.iter()) {
            for form in &letter.forms {
                if input
                    .graphemes(true)
                    .skip(index)
                    .collect::<String>()
                    .starts_with(form)
                {
                    ipa += if let Some(last) = ipa.graphemes(true).last() {
                        if last == &*letter.pronunciation {
                            "\u{2d0}"
                        } else {
                            &*letter.pronunciation
                        }
                    } else {
                        &*letter.pronunciation
                    };
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
    /// Input in conlang to translate to IPA
    input: ConlangString,
}
impl CliOptions for IpaOptions {
    fn run(&self, lang_folder: PathBuf) -> ResultAnyError<()> {
        let ortho = Orthography::from_lang_folder(&lang_folder)?;
        println!(
            "{}\n[{}]",
            White.dimmed().paint(self.input.to_owned()),
            conlang_to_ipa(self.input.to_owned(), &ortho)
        );
        Ok(())
    }
}
