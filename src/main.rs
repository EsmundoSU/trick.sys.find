mod trick_sys {
    pub mod find {
        pub struct App {
            name: String,
        }

        impl App {
            pub fn run(&self) {
                println!("App: {} started", self.name);
            }
        }

        pub struct AppBuilder {
            app: App,
        }

        impl AppBuilder {
            pub fn new(app_name: String) -> AppBuilder {
                AppBuilder {
                    app: App {
                        name: app_name,
                    }
                }
            }

            pub fn build(self) -> App {
                println!("App: {} has been build", self.app.name);
                self.app
            }
        }
    }
}

use trick_sys::find::AppBuilder;

fn main() {
    let app_builder = AppBuilder::new("tricksys-find".to_string());
    let app = app_builder.build();

    app.run();
}
