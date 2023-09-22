
use std::env;

use crate::engine::core::metadata::{ENGINE_NAME, ENGINE_VERSION};
use crate::engine::core::renderer;

#[derive(PartialEq, Clone)]
#[allow(dead_code)]
pub enum AppState {
    PreInit,
    Init,
    Running,
    Paused,
    Quitting,
}

impl ToString for AppState {
    fn to_string(&self) -> String {
        match self {
            AppState::PreInit => String::from("PreInit"),
            AppState::Init => String::from("Init"),
            AppState::Running => String::from("Running"),
            AppState::Paused => String::from("Paused"),
            AppState::Quitting => String::from("Quitting"),
        }
    }
}

#[derive(Clone)]
pub(crate) struct App {
    game_name: String,
    game_version: String,
    game_width: f64,
    game_height: f64,
    state: AppState,
    systems: Vec<(AppState, fn())>,
    debug: bool,
}
#[allow(dead_code, unused_assignments, unused_variables)]
impl App {

    pub fn new(game_name: &str, game_version: &str, game_width: f64, game_height: f64, debug: bool) -> Self {
        let args: Vec<String> = env::args().collect();
        let app = App {
            state: AppState::PreInit,
            systems: Vec::new(),
            game_name: game_name.to_owned(),
            game_version: game_version.to_owned(),
            game_width: game_width,
            game_height: game_height,
            debug: debug,
        };
        app
    }

    pub fn run(mut self) -> Self {
        let args: Vec<String> = env::args().collect();
        let mut app_state = self.state.clone();

        if app_state == AppState::PreInit {
            self.add_systems(AppState::PreInit, || {
                println!("Loading PreInit Systems...");
            });
            self.state = AppState::Init;
            app_state = self.state.clone();
        }

        if app_state == AppState::Init {
            self.add_systems(AppState::Init, || {
                println!("Loading Init Systems...");
            });
            self.state = AppState::Running;
            app_state = self.state.clone();
        }

        if self.state == AppState::Running {
            self.add_systems(AppState::Running, || {
                println!("Loading Running Systems...");
            });
        }

        // Access the command-line arguments
        // app.args[0] is the name of the program itself
        // app.args[1] is the first argument, app.args[2] is the second argument, and so on
        // You can use pattern matching or if statements to handle different arguments

        // Example: Check if the "--opengl" argument is present
        if args.contains(&String::from("--opengl")) {
            // Code to enable OpenGL
            let name = format!("{} v{}", self.game_name, self.game_version);
            renderer::core::opengl::create_opengl_window(&name, self.game_width, self.game_height);
        } else if args.contains(&String::from("--vulkano")) {
            let name = format!("{} v{} - [{} v{} - Vulkan]", self.game_name, self.game_version, ENGINE_NAME, ENGINE_VERSION);
            renderer::core::vulkano::create_vulkano_window(&name, self.game_width, self.game_height);
        } else {
            println!("No graphics API specified. Please specify a graphics API with the --opengl or --vulkano flag. Starting fallback OpenGL renderer...");
            let name = format!("{} v{}", self.game_name, self.game_version);
            renderer::core::opengl::create_opengl_window(&name, self.game_width, self.game_height);
        }
        App { 
            game_name: self.game_name, 
            game_version: self.game_version, 
            game_width: self.game_width, 
            game_height: self.game_height, 
            state: AppState::Running, 
            systems: self.systems, 
            debug: self.debug,
        }
    }

    pub fn add_systems(&mut self, state: AppState, system: fn()) {
        self.systems.push((state, system));
    }

    pub fn enable_system(&mut self) -> bool {
        // Code to run the app based on the current state and systems
        for (system_state, system) in &self.systems {
            if *system_state == self.state {
                system();
            }
        }
    
        // Set the state to Running after the window is created
        if self.state == AppState::Init {
            self.state = AppState::Running;
            true
        } else {
            false
        }
    }
}