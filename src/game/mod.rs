use crate::engine::{core::{app::App, metadata::DEBUG}, assets_loader::loader::{load_texture, list_files}};



pub fn main() {
    load_texture();
    list_files();
    let app = App::new("Test Game", " ALPHA 0.0.1", 800.0, 600.0, DEBUG);
    app.run();

}
