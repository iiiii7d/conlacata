extern crate core;

use ansi_term::Color::Red;
use clap::Parser;

mod modules;
mod types;

#[derive(Parser)]
#[clap(version, about, long_about = None)]
struct Args {
    #[clap(subcommand)]
    subcmd: Subcmd,
    /// Enables debugging info
    #[clap(short, long, parse(from_occurrences))]
    verbose: u8,
}

#[derive(Parser)]
enum Subcmd {
    #[clap(alias = "ortho")]
    Orthography(modules::orthography::OrthographyOptions),
    Ipa(modules::ipa::IpaOptions),
    #[clap(alias = "pos")]
    PartsOfSpeech(modules::parts_of_speech::PartsOfSpeechOptions),
    #[clap(alias = "lex")]
    Lexicon(modules::lexicon::LexiconOptions),
    #[clap(alias = "conj")]
    Conjugation(modules::lexicon::ConjugationOptions),
}

fn main() {
    let args = Args::parse();
    match args.subcmd {
        Subcmd::Orthography(v) => v.run(),
        Subcmd::Ipa(v) => v.run(),
        Subcmd::PartsOfSpeech(v) => v.run(),
        Subcmd::Lexicon(v) => v.run(),
        Subcmd::Conjugation(v) => v.run(),
    }.unwrap_or_else(|e| {
        eprintln!("{}", Red.paint(e.to_string()))
    })
}
