use std::path;
use std::fs;
use std::path::{PathBuf};
use crate::modules::file_system::file::File;

pub struct Directory {
    path: path::PathBuf,
    structure: fs::ReadDir,
}

impl Directory {
    pub fn new(path: String) -> Directory {
        let fs_path = PathBuf::from(&path);
        match fs::read_dir(&path) {
            Ok(dir) => { return Directory{
                path: fs_path,
                structure: dir
            } }
            Err(e) => { panic!("{}", e) }
        }
    }
    pub fn get_full_path(&self) -> &str {
        &self.path.to_str().unwrap()
    }

    pub fn contains(&self, file: File) -> bool {
        for path in self.structure {
            match path {
                Ok(path) => {
                    if path.path().to_str().unwrap() == file.get_name() { return true }
                }
                Err(error) => {panic!("{}", error)}
            }
        }
        false
    }
}


#[test]
fn read_directory_full_path() {
    // Aggregate
    let directory_name = "/home/apelc";
    // Act
    let directory = Directory::new(directory_name.to_string());
    // Assert
    assert_eq!(directory.get_full_path(), directory_name);
}

#[test]
fn directory_contains() {
    // Aggregate
    let directory_path = "assets/test/file_system_fake";
    let file = File::new("file_1.txt".to_string());
    // Act
    let directory = Directory::new(directory_path.to_string());
    // Asset
    assert!(directory.contains(file));
}
