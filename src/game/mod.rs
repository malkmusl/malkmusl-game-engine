use crate::engine::{core::{app::App, metadata::DEBUG}, assets_loader::loader::{list_files, dir_exist}};



pub fn main() {
    dir_exist();
    list_files();
    let app = App::new("Test Game", "0.0.1-alpha", 800.0, 600.0, DEBUG);
    app.run();

}
