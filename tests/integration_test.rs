use tricksys_find::app::args::Args;
use std::fs;
use clap::arg;

#[test]
fn it_works() {
    assert_eq!(2 + 2, 4);
}

#[test]
fn args_test() {
    // Scenario: find file_1.txt
    let args = Args::from(Args{ file: "file_1.txt".to_string(), search_path: "assets/test/file_system_fake".to_string()});

    // Print args
    println!("{:?}", args);

    let test = fs::read_dir(args.search_path);
    match test {
        Ok(files) => {
            for file in files {
                let file_os = file.unwrap().file_name();
                let file_name = file_os.to_str().unwrap();
                if file_name.contains(&args.file) {
                    println!("Path: {}", file_name);
                }

            }
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }


}
