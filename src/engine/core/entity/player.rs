use glium::{Surface, implement_vertex, uniform, Display};
use glutin::display;

use crate::engine::{console_logger::logger, core::renderer::core::opengl};

pub const PLAYER_DEBUG: bool = false;
pub const PLAYER_MOVEMENT_DEBUG: bool = false;

const VERTEX_SHADER_SRC: &str = r#"
    #version 140

    in vec2 position;
    uniform mat4 view;  // Uniform view matrix for aspect ratio control

    void main() {
        gl_Position = view * vec4(position, 0.0, 1.0);
    }
"#;

const FRAGMENT_SHADER_SRC: &str = r#"
    #version 140

    out vec4 color;

    void main() {
        color = vec4(1.0, 0.0, 0.0, 1.0);
    }
"#;

pub struct Player {
    pub display: glium::Display,
    pub name: String,
    pub position: [f32; 2],
    pub world_position: [f32; 2],
    velocity: [f32; 2],
    pub sprite_size: f32 ,
    program: glium::Program,
    vertex_buffer: glium::VertexBuffer<PlayerSprite>,
    index_buffer: glium::IndexBuffer<u16>,
    pub view_matrix: na::Matrix4<f32>,
}

#[derive(Copy, Clone)]
struct PlayerSprite {
    position: [f32; 2],
}

// you must pass the list of members to the macro
implement_vertex!(PlayerSprite, position);
#[allow(dead_code)]
impl Player{
    pub fn new(display: glium::Display, name: String) -> Player {

        if  opengl::is_debugging_enabled(){println!("{}", logger::info_opengl("Creating Player ShaderProgram"))};
        let program = glium::Program::from_source(&display, VERTEX_SHADER_SRC, FRAGMENT_SHADER_SRC, None).unwrap();
    
        if  opengl::is_debugging_enabled(){println!("{}", logger::info_opengl("Creating Player VertexBuffer"))};
        let vertex_buffer = glium::VertexBuffer::new(&display, &[
            PlayerSprite { position: [-0.05, -0.05] },
            PlayerSprite { position: [0.05, -0.05] },
            PlayerSprite { position: [0.05, 0.05] },
            PlayerSprite { position: [-0.05, 0.05] },
        ]).expect(&logger::error_opengl("Failed to create Player VertexBuffer"));
    
        if  opengl::is_debugging_enabled(){println!("{}", logger::info_opengl("Creating Player IndexBuffer"))};
        let index_buffer = glium::IndexBuffer::new(
            &display,
            glium::index::PrimitiveType::TriangleStrip,
            &[1 as u16, 2, 0, 3],
        ).expect(&logger::error_opengl("Failed to create Player IndexBuffer"));


        Player {
            display: display.clone(),
            name: name,
            position: [0.0, 0.0],
            world_position: [0.0, 0.0],
            velocity: [0.0, 0.0],
            sprite_size: 0.0,
            program: program,
            vertex_buffer: vertex_buffer,
            index_buffer: index_buffer,
            view_matrix:  set_view_matrix(display),
        }
        
    }

    pub fn set_position(&mut self, x: f32, y: f32) {
        self.position = [x, y];
    }

    pub fn set_velocity(&mut self, x: f32, y: f32) {
        self.velocity = [x, y];
    }

    pub fn update(&mut self, frame: &mut glium::Frame) {
        self.world_position[0] += self.velocity[0];
        self.world_position[1] += self.velocity[1];
        self.sprite_size = 0.1;
        self.set_velocity(0.0, 0.0);
        self.update_view_matrix();
        self.draw_sprite(frame);
    }

    pub fn update_camera(&mut self) -> na::Matrix4<f32> {
        // Get the current window size
        let (window_width, window_height) = self.display.get_framebuffer_dimensions();
    
        // Calculate the aspect ratio
        let aspect_ratio = window_width as f32 / window_height as f32;
    
        // Calculate the desired square size in window coordinates
        let desired_square_size = self.sprite_size / window_width as f32;
    
        // Calculate the position to center the viewport on the player
        let camera_position = na::Point2::new(
            self.world_position[0] - (window_width as f32 * 0.5 * desired_square_size),
            self.world_position[1] - (window_height as f32 * 0.5 * desired_square_size / aspect_ratio),
        );
    
        // Create a view matrix to center the viewport on the player
        let view_matrix = na::Matrix4::new_translation(&na::Vector3::new(
            -camera_position.x,
            -camera_position.y,
            0.0,
        ));
    
        view_matrix
    }

    pub fn get_view_matrix(&mut self) -> na::Matrix4<f32>{
        let matrix = self.view_matrix;
        matrix
    }

    fn update_view_matrix(&mut self) {
        self.view_matrix = set_view_matrix(self.display.clone());
    }
    
    pub fn draw_sprite(&mut self, frame: &mut glium::Frame, ) {
        //if  opengl::is_debugging_enabled(){println!("{}", logger::info_opengl("Setting Player Uniforms"))};
        // Set the sprite_size uniform
        let uniforms = uniform! {
            view: *self.view_matrix.as_ref(),
        };
        frame.draw(
            &self.vertex_buffer,
            &self.index_buffer,
            &self.program,
            &uniforms,
            &Default::default(),
        ).expect(&logger::error_opengl("Failed to draw Player to Frame"));
    }

    pub fn handle_input(&mut self, event: &mut glium::glutin::event::WindowEvent) {
        match event {
            glium::glutin::event::WindowEvent::KeyboardInput { input, .. } => {
                if let Some(keycode) = input.virtual_keycode {
                    match keycode {
                        glium::glutin::event::VirtualKeyCode::W => {logger::debug_player_movement(self, "W"); self.velocity[1] = 0.01},
                        glium::glutin::event::VirtualKeyCode::A => {logger::debug_player_movement(self, "A"); self.velocity[0] = -0.01},
                        glium::glutin::event::VirtualKeyCode::S => {logger::debug_player_movement(self, "S"); self.velocity[1] = -0.01},
                        glium::glutin::event::VirtualKeyCode::D => {logger::debug_player_movement(self, "D"); self.velocity[0] = 0.01},
                        _ => (),
                    }
                }
            }
            _ => (),
        }
    }
    
}


pub fn set_view_matrix(display: Display)  -> na::Matrix4<f32>{
    // Get the current window size
    let (window_width, window_height) = display.get_framebuffer_dimensions();

    // Calculate the aspect ratio
    let aspect_ratio = window_width as f32 / window_height as f32;

    // Calculate the desired square size in window coordinates

    // Create a view matrix to control the aspect ratio
    let view_matrix = na::Matrix4::new(
        1.0, 0.0, 0.0, 0.0,
        0.0, aspect_ratio, 0.0, 0.0,
        0.0, 0.0, 1.0, 0.0,
        0.0, 0.0, 0.0, 1.0,
    ); 
    view_matrix
}