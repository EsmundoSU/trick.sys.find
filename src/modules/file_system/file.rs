pub struct File {
    name: String,
}

impl File {
    pub fn new(name: String) -> File {
        File {
            name,
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }
}

#[test]
fn create_file() {
    // Aggregate
    let file_name = "test_file";
    // Act
    let file = File::new(file_name.to_string());
    // Assert
    assert_eq!(file_name, file.get_name());
}
