extern crate core;

use std::path::PathBuf;
use ansi_term::Color::Red;
use clap::Parser;
use crate::types::ResultAnyError;

mod modules;
mod types;

#[derive(Parser)]
#[clap(version, about, long_about = None)]
struct Args {
    /// The language folder to operate on
    lang_folder: PathBuf,
    #[clap(subcommand)]
    subcmd: Subcmd,
    /// Enables debugging info
    #[clap(short, long, parse(from_occurrences))]
    verbose: u8,
}

#[derive(Parser)]
enum Subcmd {
    /// View the language's orthography
    #[clap(alias = "ortho")]
    Orthography(modules::orthography::OrthographyOptions),
    /// Translate a sentence in a conlang to IPA
    Ipa(modules::ipa::IpaOptions),
    /// View the language's parts of speech
    #[clap(alias = "pos")]
    PartsOfSpeech(modules::parts_of_speech::PartsOfSpeechOptions),
    /// View the language's lexicon
    #[clap(alias = "lex")]
    Lexicon(modules::lexicon::LexiconOptions),
    /// Conjugate a word in the language
    #[clap(alias = "conj")]
    Conjugation(modules::lexicon::ConjugationOptions),
}

pub trait CliOptions {
    fn run(&self, lang_folder: PathBuf) -> ResultAnyError<()>;
}

fn main() {
    let args = Args::parse();
    match args.subcmd {
        Subcmd::Orthography(v) => v.run(args.lang_folder),
        Subcmd::Ipa(v) => v.run(args.lang_folder),
        Subcmd::PartsOfSpeech(v) => v.run(args.lang_folder),
        Subcmd::Lexicon(v) => v.run(args.lang_folder),
        Subcmd::Conjugation(v) => v.run(args.lang_folder),
    }.unwrap_or_else(|e| {
        eprintln!("{}", Red.paint(e.to_string()))
    })
}
