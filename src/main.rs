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
    PartsOfSpeech(modules::parts_of_speech::PartsOfSpeechOptions)
}

fn main() {
    let args = Args::parse();
    match args.subcmd {
        Subcmd::Orthography(v) => v.run(),
        Subcmd::Ipa(v) => v.run(),
        Subcmd::PartsOfSpeech(v) => v.run(),
    }
}
