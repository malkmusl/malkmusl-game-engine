use engine::core::app::{App, AppState};
mod engine;


fn main() {

    let mut app = App::new("Test Game", " ALPHA 0.0.1", 800.0, 600.0);
    app.add_systems(AppState::Running, test_system);
    app.run();

}

pub fn test_system() {
    println!("Loading Test Systems...");
}
