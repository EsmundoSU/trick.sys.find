use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[clap(author = "Adam Pelc <adam.pelc.su@gmail.com>")]
#[clap(about = "TBD")]
#[clap(author, version)]
pub struct Args {
    /// Name of file to be found
    #[clap(short, long)]
    pub file: String,

    /// Search path
    #[clap(short, long)]
    pub search_path: String,
}
