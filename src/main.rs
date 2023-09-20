use engine::core::app::{App, AppState};
mod engine;
use engine::core::time::macros::*;


fn main() {

    let mut app = App::new("Test Game", " ALPHA 0.0.1", 800.0, 600.0);
    app.run();

}
