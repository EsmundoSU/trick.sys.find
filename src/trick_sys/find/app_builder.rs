use super::app::App;

pub struct AppBuilder {
    app: App,
}

impl AppBuilder {
    pub fn new(app_name: &str) -> AppBuilder {
        AppBuilder {
            app: App::new(app_name)
        }
    }

    pub fn build(self) -> App {
        println!("App: {} has been build", self.app.get_name());
        self.app
    }
}

