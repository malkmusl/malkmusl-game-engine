extern crate glium;
extern crate lazy_static;

use lazy_static::lazy_static;

use glium::{Frame, Display};
use crate::engine::assets_loader;
use crate::engine::assets_loader::texture_tilesets::OUTSIDE_ATLAS;
use crate::engine::assets_loader::texture_loader::TextureAtlas;
use crate::engine::console_logger::logger::{self, set_color};
use crate::engine::core::entity::npc;
use crate::engine::core::metadata::{ENGINE_NAME, ENGINE_VERSION, VSYNC, COLOR_CYAN, self};
use crate::engine::core::entity::player;
use crate::engine::core::renderer::d2::background_tiles;
use crate::engine::core::renderer::camera::camera2d;

use super::GameStatus;

lazy_static! {
    pub static ref OPENGL_PREFIX: String = {
        let prefix = "[Renderer - OpenGL]";
        set_color(COLOR_CYAN, prefix)
    };
}

struct OpenGLDisplay{
    display: Display
}

pub static mut OPENGL_DEBUG: bool = true;

/// Creates an OpenGL window for the game with the specified name, width, and height.
/// This function sets up the necessary event loop, window parameters, OpenGL context,
/// and initializes game entities such as the player and NPCs. It also handles various
/// events like window close requests and keyboard input for game control.
///
/// # Arguments
///
/// * `game_name` - The name of the game.
/// * `game_width` - The width of the game window.
/// * `game_height` - The height of the game window.
///
/// # Example
///
/// ```rust
/// create_opengl_window("MyGame", 800.0, 600.0);
/// ```
///
/// This function encapsulates the setup and main loop of the game's graphical interface.
/// It is a crucial part of initializing and running the game.

#[allow(unused_mut)]
pub fn create_opengl_window(game_name: &str, game_width: f64, game_height: f64) {
    let graphics_api = "OpenGL";
    let mut state = GameStatus::Running;
    let engine_verison: &str = &*ENGINE_VERSION;

    let app_name = game_name.to_owned() + " - [" + ENGINE_NAME + " v"+engine_verison+ " - "+ graphics_api+"]"; 
    // 1. The **winit::EventsLoop** for handling events.
    if is_debugging_enabled(){println!("{}", logger::info_opengl("Creating EventLoop"))};
    let mut events_loop = glium::glutin::event_loop::EventLoop::new();
    // 2. Parameters for building the Window.
    if is_debugging_enabled(){println!("{}", logger::info_opengl("Creating WindowBuilder"))};
    let wb = glium::glutin::window::WindowBuilder::new()
        .with_inner_size(glium::glutin::dpi::LogicalSize::new(game_width, game_height))
        .with_title(app_name);
    // 3. Parameters for building the OpenGL context.
    if is_debugging_enabled(){println!("{}", logger::info_opengl("Creating ContextBuffer"))};
    let cb = glium::glutin::ContextBuilder::new().with_vsync(VSYNC);
    // 4. Build the Display with the given window and OpenGL context parameters and register the
    //    window with the events_loop.
    if is_debugging_enabled(){println!("{}", logger::info_opengl("Creating Display"))};
    let display = glium::Display::new(wb, cb, &events_loop).expect(&logger::error_opengl("Failed to create Display"));

    let mut player = player::Player::new(display.clone(), "makmusl".to_string());
    let mut npc = npc::NPC::new(display.clone());
    
    // 5. start EventsLoop
    println!("{}", logger::warn_opengl("Starting EventLoop"));
    events_loop.run(move |event, _, control_flow| {
        // 6. Handle events here
        match event {
            glium::glutin::event::Event::WindowEvent { mut event, .. } => {
                match event {
                    glium::glutin::event::WindowEvent::CloseRequested => {
                        *control_flow = glium::glutin::event_loop::ControlFlow::Exit;
                        state = GameStatus::Stopped;
                        logger::game_state(state, 0);
                    }
                    _ => {
                        if state == GameStatus::Running {
                            player.handle_input(&mut event);
                            npc.handle_input(&mut event);
                        }
                    }
                }
            },
            glium::glutin::event::Event::DeviceEvent { event, .. } => {
                match event {
                    glium::glutin::event::DeviceEvent::Key(input) => {
                        match input.virtual_keycode {
                            Some(glium::glutin::event::VirtualKeyCode::Escape) => {
                                if input.state == glium::glutin::event::ElementState::Pressed {
                                    if state == GameStatus::Running{
                                        state = GameStatus::Paused;
                                        logger::game_state(state, 21);
                                    }else{
                                        state = GameStatus::Running;
                                        logger::game_state(state, 22);
                                    }
                                }
                            }
                            _ => (),
                        }
                    }
                    _ => (),
                }
            },
            _ => {
                if state == GameStatus::Running {
                    *control_flow = glium::glutin::event_loop::ControlFlow::Poll;
                    update_content(display.clone(), &mut player, &mut npc);
                } else {
                    *control_flow = glium::glutin::event_loop::ControlFlow::Wait;
                }
            },
        }        
    });
}

/// Updates the content of the game's display. This function is responsible for drawing
/// the game's elements, including the player and NPCs, onto the display, updating the
/// background tiles, and finishing the frame.
///
/// # Arguments
///
/// * `display` - A reference to the glium display where the game is rendered.
/// * `player` - A mutable reference to the player object to update and draw.
/// * `npc` - A mutable reference to the NPC (non-player character) object to update and draw.
///
/// This function performs the necessary drawing and updating operations, culminating
/// in finishing the frame, making the changes visible on the game window.
///
/// # Example
///
/// ```rust
/// let display = /* initialize your glium display */;
/// let mut player = /* initialize your player */;
/// let mut npc = /* initialize your NPC */;
/// update_content(display, &mut player, &mut npc);
/// ```
///
/// This function is a key part of the game loop, responsible for rendering and updating
/// the visual elements of the game.

pub fn update_content(display: glium::Display,player: &mut player::Player, _npc: &mut npc::NPC){
    let mut frame = display.draw();
    //draw_squareV2(display.clone(), &mut frame);
    update_background_tiles(display.clone(), &mut frame, player);
    update_player(player, &mut frame);
    //update_npc(npc, &mut frame);
    frame.finish().expect(&logger::error_opengl("Failed to finish Frame"));
}

/// Updates the player object and its interaction with the game frame. This function
/// is responsible for advancing the player's state and handling camera adjustments
/// to follow the player's position.
///
/// # Arguments
///
/// * `player` - A mutable reference to the player object to update.
/// * `frame` - A mutable reference to the game frame where updates take place.
///
/// This function calls the `update` method on the player, allowing it to process
/// its logic and animations. It also ensures that the camera follows the player's
/// position, keeping them centered within the frame.
///
/// # Example
///
/// ```rust
/// let mut player = /* initialize your player */;
/// let mut frame = /* initialize your game frame */;
/// update_player(&mut player, &mut frame);
/// ```
///
/// This function plays a crucial role in keeping the player character up-to-date and
/// managing the in-game camera's behavior.

pub fn update_player(player: &mut player::Player, frame: &mut Frame) {
    player.update(frame);
    camera2d::update_camera_follow_player(player, frame, player.position)
}

/// Updates the NPC (non-player character) object within the game frame. This function
/// is responsible for advancing the NPC's state and interactions with the game world.
///
/// # Arguments
///
/// * `npc` - A mutable reference to the NPC object to update.
/// * `frame` - A mutable reference to the game frame where updates take place.
///
/// This function calls the `update` method on the NPC, allowing it to process its logic,
/// animations, and interactions with the game world.
///
/// # Example
///
/// ```rust
/// let mut npc = /* initialize your NPC */;
/// let mut frame = /* initialize your game frame */;
/// update_npc(&mut npc, &mut frame);
/// ```
///
/// This function is essential for keeping the NPC character up-to-date within the game world.

#[allow(dead_code)]
pub fn update_npc(npc: &mut npc::NPC, mut frame: &mut Frame) {
    npc.update(&mut frame);
}

/// Updates the background tiles within the game frame. This function is responsible for
/// rendering and updating the background elements, such as tiles or grids, to create the
/// game's visual environment.
///
/// # Arguments
///
/// * `display` - A glium display reference where the background tiles are drawn.
/// * `frame` - A mutable reference to the game frame where updates take place.
///
/// This function typically calls specific drawing functions or modules to create the
/// background tiles or grids, setting the visual backdrop for the game world.
///
/// # Example
///
/// ```rust
/// let display = /* initialize your glium display */;
/// let mut frame = /* initialize your game frame */;
/// update_background_tiles(display, &mut frame);
/// ```
///
/// This function plays a crucial role in maintaining the visual aspect of the game's
/// environment.

pub fn update_background_tiles(display: glium::Display, frame: &mut Frame, player: &mut player::Player){
    //background_tiles::draw(display.clone(), frame, 10, 10, 0.5);
    //testing::simple_square::draw_square_grid(&display, frame, 1, 3, 0.2);
    let _ = assets_loader::loader::load_tiles_from_file("test");

    let texture = assets_loader::loader::load_texture(&display, "moss_block.png");

    //let atlas_texture = OUTSIDE_ATLAS.load_texture_from_atlas([1, 1], display.clone());

    let mut background = background_tiles::BackgroundTiles::new(display);
    background.draw(frame, 10, 10, 0.1, &texture, player);
    

    // Call the draw_square_grid_with_texture function with the loaded texture
    //testing::simple_square::draw_square_grid_with_texture(&display, frame, 5, 5, 0.2, &texture);
    //testing::simple_square::draw_square_grid_with_texture_and_player(&display, frame, 32, 32, 0.1, &texture, player);
    /*
    let mut layer_0 = background_tiles::BackgroundTiles::new(display.clone());
    let tile_0 = background_tiles::Tile::new([0.0,0.0], 0.1, texture);
    let tile_1= background_tiles::Tile::new([1.0, 1.0], 0.1, texture2);
    layer_0.add_tile(tile_0);
    layer_0.add_tile(tile_1);
    layer_0.draw(frame, player); 
    */
    
    
}


pub struct Level{
    pub data: Vec<background_tiles::BackgroundTiles>
}

pub fn load_background() {
    let tileset_name = "outside"; // replace with the actual logic to determine the tileset name

    if let Some(atlas) = get_atlas_by_name(tileset_name) {
        // Atlas with the specified name exists, load a texture
        // Do something with atlas_texture...
    } else {
        // Atlas with the specified name does not exist
        println!("Atlas with name {} not found", tileset_name);
    }
}
fn get_atlas_by_name(name: &str) -> Option<&TextureAtlas> {
    // Implement your logic to get the atlas by name
    // Return Some(atlas) if found, None otherwise
    // For example:
    match name {
        "outside" => Some(&OUTSIDE_ATLAS),
        // Add more cases for other atlas names if needed
        _ => None,
    }
}


pub fn is_debugging_enabled() -> bool {
    if metadata::DEBUG || unsafe { OPENGL_DEBUG } {
        return true;
    }else{
        return false;
    }
}