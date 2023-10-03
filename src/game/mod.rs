use crate::engine::core::{app::App, metadata::DEBUG};



pub fn main() {
    //dir_exist();
    //list_files();
    let app = App::new("Test Game", "0.0.1-alpha", 800, 600, DEBUG);
    app.run();

}
