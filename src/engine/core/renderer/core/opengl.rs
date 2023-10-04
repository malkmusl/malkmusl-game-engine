extern crate glium;
extern crate lazy_static;

use winit::window::Fullscreen;
use glium::glutin::{ContextBuilder, NotCurrent};
use lazy_static::lazy_static;


use glium::{Frame, Display};
use tokio::sync::Mutex;
use winit::dpi::LogicalSize;

use winit::window::{WindowBuilder, Icon};
use crate::engine::assets_loader;
use crate::engine::assets_loader::loader::ASSET_FOLDER;

use crate::engine::assets_loader::texture_loader::TextureAtlas;
use crate::engine::assets_loader::texture_tilesets::OUTSIDE_ATLAS;
use crate::engine::console_logger::logger::{self, set_color};
use crate::engine::core::entity::npc;
use crate::engine::core::metadata::{ENGINE_NAME, ENGINE_VERSION, VSYNC, COLOR_CYAN, self};
use crate::engine::core::entity::player;
use crate::engine::core::renderer::d2::background_tiles;
use crate::engine::core::renderer::camera::camera2d;
use std::sync::Arc;

use image::{ImageBuffer, Rgba};
use std::collections::HashMap;

use super::GameStatus;

lazy_static! {
    pub static ref OPENGL_PREFIX: String = {
        let prefix = "[Renderer - OpenGL]";
        set_color(COLOR_CYAN, prefix)
    };
}



pub static mut OPENGL_DEBUG: bool = true;
pub struct OpenGLWindow {
    event_loop: glium::glutin::event_loop::EventLoop<()>,
    _wb: Arc<WindowBuilder>,
    _cb: Arc<ContextBuilder<'static, NotCurrent>>,
    display: Arc<Display>,
    _shared_state: Arc<Mutex<u32>>,// To enforce lifetime constraints
}

impl<'a> OpenGLWindow {
    pub fn new(game_width: u32, game_height: u32, app_name: &str, vsync: bool) -> Self {
        let graphics_api = "OpenGL";
        let engine_version: &str = &*ENGINE_VERSION;
        let app_name = format!("{} - [{} v{} - {}]", app_name, ENGINE_NAME, engine_version, graphics_api);

        if is_debugging_enabled() {
            println!("{}", logger::info_opengl("Creating EventLoop"));
        }
        let event_loop = glium::glutin::event_loop::EventLoop::new();

        if is_debugging_enabled() {
            println!("{}", logger::info_opengl("Creating WindowBuilder"));
        }
        let wb = WindowBuilder::new()
            .with_inner_size(LogicalSize::new(game_width, game_height))
            .with_title(app_name);
        Self::set_icon(wb.clone());

        if is_debugging_enabled() {
            println!("{}", logger::info_opengl("Creating ContextBuffer"));
        }

        let cb = ContextBuilder::new().with_vsync(vsync);
        if is_debugging_enabled() {
            println!("{}", logger::info_opengl("Creating Display"));
        }
        let display = Display::new(wb.clone(), cb.clone(), &event_loop)
            .unwrap_or_else(|e| panic!("{}", logger::error_opengl(&format!("Failed to create Display: {}", e))));

        let shared_state = Arc::new(Mutex::new(0u32));

        OpenGLWindow {
            event_loop: event_loop,
            _wb: Arc::new(wb),
            _cb: Arc::new(cb),
            display: Arc::new(display),
            _shared_state: shared_state,
        }
    }

    pub fn get_display(&self) -> Display {
        let display = Arc::clone(&self.display);
        (*display).clone()
    }

    pub fn get_event_loop(self) -> glium::glutin::event_loop::EventLoop<()> {
        let event_loop = self.event_loop;
        event_loop
    }

    #[allow(dead_code)]
    pub fn get_shared_state(&self) -> &Arc<Mutex<u32>> {
        &self._shared_state
    }

pub fn set_icon(wb: WindowBuilder) {
    // Load the PNG file

    let icon_path = format!("{}/apple.png", ASSET_FOLDER);
    let icon_image = image::open(icon_path).expect("Failed to open icon image");

    // Convert the image to RGBA format
    let rgba_image = icon_image.to_rgba8();

    // Get the image dimensions
    let width = rgba_image.width();
    let height = rgba_image.height();


    // Create a winit::window::Icon from the texture
    let icon = Icon::from_rgba(rgba_image.clone().to_vec(), width, height)
        .expect("Failed to create icon from texture");

    // Set the window icon and return the new WindowBuilder
    let _ = wb.with_window_icon(Some(icon));
}

// Add other methods as needed
}


#[allow(unused_mut)]
pub fn create_opengl_window(game_name: &str, game_width: u32, game_height: u32) {
    let mut state = GameStatus::Running;
    let mut is_fullscreen = false;

    let gl_window = OpenGLWindow::new(game_width, game_height, game_name, true);
    let display = gl_window.get_display();
    let d = display.clone();
    let window = d.gl_window();
    let event_loop = gl_window.get_event_loop();


    load_atlases(display.clone());
    //output_textures(&OUTSIDE_ATLAS.lock().unwrap().textures);

    let mut player = player::Player::new(display.clone(), "makmusl".to_string());
    let mut npc = npc::NPC::new(display.clone());

    // 5. start EventsLoop
    println!("{}", logger::warn_opengl("Starting EventLoop"));
    event_loop.run(move |event, _, control_flow| {
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
                            },
                            Some(glium::glutin::event::VirtualKeyCode::F11) => {
                                if input.state == glium::glutin::event::ElementState::Pressed {
                                    is_fullscreen = !is_fullscreen;
                                    let new_state = if is_fullscreen {
                                        Some(Fullscreen::Borderless(
                                            display.gl_window().window().primary_monitor(),
                                        ))
                                    } else {
                                        None
                                    };
                                    display.gl_window().window().set_fullscreen(new_state);
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
    background.draw(frame, 10, 10, player);
    

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

#[allow(dead_code)]
pub struct Level{
    pub data: Vec<background_tiles::BackgroundTiles>
}


fn load_atlases(display: glium::Display){
    let outside_atlas = OUTSIDE_ATLAS.lock().expect("Failed to lock OUTSIDE_ATLAS").load_texture_from_tileset_to_map(&display);
}


fn output_textures(textures: &HashMap<u32, ImageBuffer<Rgba<u8>, Vec<u8>>>) {
    for (key, image_buffer) in textures {
        // Process or print the key and image_buffer as needed
        println!("Key: {}", key);

        // For demonstration, let's print the dimensions of the image buffer
        let (width, height) = image_buffer.dimensions();
        println!("Image Dimensions: {} x {}", width, height);

        // You can perform other operations on the image_buffer as needed
        // ...

        // Example: Access a pixel value

        // Example: Iterate over all pixels
        for (_, _, pixel) in image_buffer.enumerate_pixels() {
            // Process each pixel as needed
            // ...
        }
    }
}



pub fn is_debugging_enabled() -> bool {
    if metadata::DEBUG || unsafe { OPENGL_DEBUG } {
        return true;
    }else{
        return false;
    }
}