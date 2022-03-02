use super::search_engine;

pub struct App{
    name: String,
}

impl App {
    pub fn new(name: &str) -> App {
        App {
            name: name.to_string(),
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn run(&self) {
        println!("App: {} started", self.name);

        search_engine::find_file("temp", "/root/home")
    }
}
