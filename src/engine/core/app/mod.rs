
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

/// The `App` struct represents the core of a game application, managing its state and systems.
///
/// This implementation provides essential functionality for creating, configuring, and running a game application.
/// It allows you to initialize the game environment, register system functions for different application states,
/// and execute those systems   based on the current state. Additionally, it handles the selection of graphics APIs
/// via command-line arguments and transitions the application state from `PreInit` to `Init` and finally to `Running`.
///
/// # Examples
///
/// ```
/// // Create an instance of the game application
/// let mut game = App::new("MyGame", "1.0", 800.0, 600.0, true);
///
/// // Register and execute system functions
/// game.add_systems(AppState::Init, || {
///     // Initialization logic goes here
/// });
///
/// let state_transition = game.enable_system();
///
/// if state_transition {
///     // The state transition from Init to Running occurred
///     // Perform additional actions here if needed
/// }
///
/// // Run the game application
/// let updated_game = game.run();
/// ```
///
/// In this example, the `App` struct is used to create and manage a game application, including initializing systems
/// and running the application loop.

#[derive(Clone)]
pub(crate) struct App {
    game_name: String,
    game_version: String,
    game_width: u32,
    game_height: u32,
    state: AppState,
    systems: Vec<(AppState, fn())>,
    debug: bool,
}
#[allow(dead_code, unused_assignments, unused_variables)]
impl App {

    /// Creates a new instance of the `App` struct with the specified parameters.
    ///
    /// # Arguments
    ///
    /// * `game_name` - A string representing the name of the game.
    /// * `game_version` - A string representing the version of the game.
    /// * `game_width` - The width of the game window as a floating-point number.
    /// * `game_height` - The height of the game window as a floating-point number.
    /// * `debug` - A boolean indicating whether debug mode is enabled.
    ///
    /// # Returns
    ///
    /// A new `App` instance initialized with the provided values and default state.

    pub fn new(game_name: &str, game_version: &str, game_width: u32, game_height: u32, debug: bool) -> Self {
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

    /// Runs the game application, initializing systems based on the current application state and command-line arguments.
    ///
    /// This function sets up the game environment, initializes necessary systems, and creates the game window based on the specified or default graphics API.
    /// It also transitions the application state from `PreInit` to `Init` and finally to `Running` as the game progresses.
    ///
    /// # Returns
    ///
    /// A new `App` instance representing the updated game application state after running.
    ///
    /// # Remarks
    ///
    /// - The function checks for command-line arguments to determine the graphics API to use (`--opengl` or `--vulkano`) or defaults to OpenGL if none are specified.
    /// - Different systems are initialized based on the current application state, and messages are printed to indicate the loading process.
    /// - The `AppState` enum is used to manage the application state transitions.
    ///
    /// # Example
    ///
    /// ```rust
    /// let game = App::new("MyGame", "1.0", 800.0, 600.0, true);
    /// let updated_game = game.run();
    /// ```

    pub fn run(mut self) -> Self {
        let args: Vec<String> = env::args().collect();
        let mut app_state = self.state.clone();
        let engine_verison: &str = &*ENGINE_VERSION;

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
            let name = format!("{} v{} - [{} v{} - Vulkan]", self.game_name, self.game_version, ENGINE_NAME, engine_verison);
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

    /// Adds a system function to the game application for a specific application state.
    ///
    /// This function allows you to register a system function to be executed when the game transitions
    /// to a specific application state. Systems are functions that perform tasks or updates related to
    /// specific game states, such as initialization or running the game loop.
    ///
    /// # Arguments
    ///
    /// * `state` - The target application state for which the system function will be executed.
    /// * `system` - A function that represents the system to be executed when the specified state is reached.
    ///
    /// # Example
    ///
    /// ```rust
    /// // Define a custom system function
    /// fn custom_init_system() {
    ///     // Initialization logic goes here
    /// }
    ///
    /// // Create an instance of the game application
    /// let mut game = App::new("MyGame", "1.0", 800.0, 600.0, true);
    ///
    /// // Register the custom system to run during the Init state
    /// game.add_systems(AppState::Init, custom_init_system);
    /// ```
    ///
    /// In this example, the `custom_init_system` function will be executed when the game transitions to the `Init` state.

    pub fn add_systems(&mut self, state: AppState, system: fn()) {
        self.systems.push((state, system));
    }

    /// Executes registered system functions based on the current application state.
    ///
    /// This function iterates through the registered system functions and executes the one associated
    /// with the current application state. It is typically called to perform specific tasks or updates
    /// during different stages of the game, such as initialization or running the game loop.
    ///
    /// # Returns
    ///
    /// A boolean indicating whether the state transition from `Init` to `Running` occurred during the execution.
    ///
    /// # Example
    ///
    /// ```rust
    /// // Create an instance of the game application
    /// let mut game = App::new("MyGame", "1.0", 800.0, 600.0, true);
    ///
    /// // Register and execute system functions
    /// game.add_systems(AppState::Init, || {
    ///     // Initialization logic goes here
    /// });
    ///
    /// let state_transition = game.enable_system();
    ///
    /// if state_transition {
    ///     // The state transition from Init to Running occurred
    ///     // Perform additional actions here if needed
    /// }
    /// ```
    ///
    /// In this example, the `enable_system` function executes registered system functions based on the
    /// current state, and it returns `true` when transitioning from `Init` to `Running`.

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