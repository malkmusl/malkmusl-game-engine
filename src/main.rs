use engine::core::app::App;
mod engine;
use engine::core::metadata::DEBUG;


pub fn main() {

    let app = App::new("Test Game", " ALPHA 0.0.1", 800.0, 600.0, DEBUG);
    app.run();

}
