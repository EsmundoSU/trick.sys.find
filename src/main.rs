pub mod trick_sys;

use trick_sys::find::app_builder::{AppBuilder};

fn main() {
    let app_builder = AppBuilder::new("tricksys-find");
    let app = app_builder.build();

    app.run();
}
