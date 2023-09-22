use glium::{Surface, implement_vertex, uniform};

use crate::engine::console_logger::Logger;

pub const PLAYER_DEBUG: bool = false;
pub const PLAYER_MOVEMENT_DEBUG: bool = false;

#[derive(Clone)]
pub struct Player {
    display: glium::Display,
    pub name: String,
    pub position: [f32; 2],
    velocity: [f32; 2],
    pub sprite_size: [f32; 2],
}

#[derive(Copy, Clone)]
struct PlayerSprite {
    position: [f32; 2],
}

// you must pass the list of members to the macro
implement_vertex!(PlayerSprite, position);

impl Player{
    pub fn new(display: glium::Display, name: String) -> Player {
        Player {
            display: display,
            name: name,
            position: [0.0, 0.0],
            velocity: [0.0, 0.0],
            sprite_size: [0.0, 0.0],
        }
        
    }

    pub fn set_position(&mut self, x: f32, y: f32) {
        self.position = [x, y];
    }

    pub fn set_velocity(&mut self, x: f32, y: f32) {
        self.velocity = [x, y];
    }

    pub fn update(&mut self, frame: &mut glium::Frame) {
        self.position[0] += self.velocity[0];
        self.position[1] += self.velocity[1];
        self.sprite_size = [0.5, 0.5];
        self.draw_sprite();
    }

    pub fn draw_sprite(&mut self, frame: &mut glium::Frame) {
        let half_size = self.sprite_size[0] / 2.0;
        let vertex_buffer = glium::VertexBuffer::new(&self.display, &[
            PlayerSprite { position: [-half_size + self.position[0], -half_size + self.position[1]] },
            PlayerSprite { position: [half_size + self.position[0], -half_size + self.position[1]] },
            PlayerSprite { position: [half_size + self.position[0], half_size + self.position[1]] },
            PlayerSprite { position: [-half_size + self.position[0], half_size + self.position[1]] },
        ]).unwrap();
    
        let index_buffer = glium::IndexBuffer::new(
            &self.display,
            glium::index::PrimitiveType::TriangleStrip,
            &[1 as u16, 2, 0, 3],
        ).unwrap();
    
        let vertex_shader_src = r#"
            #version 140

            in vec2 position;
            uniform vec2 spriteSize;

            void main() {
                vec2 scaledPosition = position * spriteSize;
                gl_Position = vec4(scaledPosition, 0.0, 2.0);
            }
        "#;
    
        let fragment_shader_src = r#"
            #version 140
    
            out vec4 color;
    
            void main() {
                color = vec4(1.0, 0.0, 1.0, 1.0);
            }
        "#;
    
        let program = glium::Program::from_source(&self.display, vertex_shader_src, fragment_shader_src, None).unwrap();
    
        // Set the sprite_size uniform
        let uniforms = uniform! {
            spriteSize: self.sprite_size,
        };
        frame.draw(
            &vertex_buffer,
            &index_buffer,
            &program,
            &uniforms,
            &Default::default(),
        ).unwrap();
    }

    pub fn handle_input(&mut self, event: &mut glium::glutin::event::WindowEvent) {
        match event {
            glium::glutin::event::WindowEvent::KeyboardInput { input, .. } => {
                if let Some(keycode) = input.virtual_keycode {
                    match keycode {
                        glium::glutin::event::VirtualKeyCode::W => {Logger::debug_player_movement(self, "W"); self.velocity[1] = 0.1},
                        glium::glutin::event::VirtualKeyCode::A => {Logger::debug_player_movement(self, "A"); self.velocity[0] = -0.1},
                        glium::glutin::event::VirtualKeyCode::S => {Logger::debug_player_movement(self, "S"); self.velocity[1] = -0.1},
                        glium::glutin::event::VirtualKeyCode::D => {Logger::debug_player_movement(self, "D"); self.velocity[0] = 0.1},
                        _ => (),
                    }
                }
            }
            _ => (),
        }
    }
    
}

