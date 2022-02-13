use clap::{App, AppSettings, Parser};

#[derive(Parser)]
struct Args {
    #[clap(long)]
    search_type: String,

    #[clap(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn main() {
    let _app = App::new("tricksys-find")
        .author("Adam Pelc, adam.pelc.su@gmail.com")
        .version("0.1.0")
        .about("Versatile system utility designed to: search for files or search the contents of files in a simple and intuitive way.")
        .bin_name("tricksys-find")
        .setting(AppSettings::SubcommandRequired)
        .subcommand(App::new("file").about("Search type: is looking for single file"))
        .subcommand(App::new("files").about("Search type: is looking for multiple files"))
        .subcommand(App::new("content").about("Search type: is looking for some content is file(s)"));
}
