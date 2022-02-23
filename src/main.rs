

use trick_sys::find::AppBuilder;

fn main() {

    let _app_builder = AppBuilder.build();


    // let _app = Command::new("tricksys-find")
    //     .author("Adam Pelc, adam.pelc.su@gmail.com")
    //     .version("0.1.0")
    //     .about("Versatile system utility designed to: search for files or search the contents of files in a simple and intuitive way.")
    //     .bin_name("tricksys-find")
    //     .arg(
    //         Arg::new("files")
    //             .short('f')
    //             .long("files")
    //             .takes_value(true)
    //             .multiple_values(true)
    //             .conflicts_with("content")
    //     )
    //     .arg(
    //         Arg::new("content")
    //             .short('c')
    //             .long("content")
    //             .takes_value(true)
    //             .multiple_values(true)
    //             .conflicts_with("files")
    //     )
    //     .arg(
    //         Arg::new("in")
    //             .short('i')
    //             .long("in")
    //             .takes_value(true)
    //             .multiple_values(true)
    //     );

        // AppBuilder app_builder.default_cli();
        // app_builder.build();
}
