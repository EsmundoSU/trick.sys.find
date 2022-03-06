
use crate::modules::file_system::{directory::Directory, file::File};

pub fn find_file(file: File, directory: Directory) {
    directory.contains(file);
}
