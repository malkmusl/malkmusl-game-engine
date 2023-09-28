use glium::{Surface, implement_vertex, uniform};

use crate::engine::console_logger::Logger;



#[derive(Clone)]
pub struct NPC {
    display: glium::Display,
    position: [f32; 2],
    velocity: [f32; 2],
    pub sprite_size: f32,
}

#[derive(Copy, Clone)]
struct NPCSprite {
    position: [f32; 2],
}

// you must pass the list of members to the macro
implement_vertex!(NPCSprite, position);

impl NPC{
    pub fn new(display: glium::Display) -> NPC {
        NPC {
            display: display,
            position: [0.0, 0.0],
            velocity: [0.0, 0.0],
            sprite_size: 0.0,
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
        self.sprite_size = 0.1;
        self.set_velocity(0.0, 0.0);
        self.draw_sprite(frame);
    }

    pub fn draw_sprite(&mut self, frame: &mut glium::Frame) {

        // Get the current window size
        let (window_width, window_height) = self.display.get_framebuffer_dimensions();

        // Calculate the aspect ratio
        let aspect_ratio = window_width as f32 / window_height as f32;

        // Calculate the desired square size in window coordinates
        let desired_square_size = self.sprite_size / window_width as f32;

        // Create a view matrix to control the aspect ratio
        let view_matrix = na::Matrix4::new(
            1.0, 0.0, 0.0, 0.0,
            0.0, aspect_ratio, 0.0, 0.0,
            0.0, 0.0, 1.0, 0.0,
            0.0, 0.0, 0.0, 1.0,
        );


        let half_size = self.sprite_size / 2.0;
        let vertex_buffer = glium::VertexBuffer::new(&self.display, &[
            NPCSprite { position: [-half_size + self.position[0], -half_size + self.position[1]] },
            NPCSprite { position: [half_size + self.position[0], -half_size + self.position[1]] },
            NPCSprite { position: [half_size + self.position[0], half_size + self.position[1]] },
            NPCSprite { position: [-half_size + self.position[0], half_size + self.position[1]] },
        ]).unwrap();
    
        let index_buffer = glium::IndexBuffer::new(
            &self.display,
            glium::index::PrimitiveType::TriangleStrip,
            &[1 as u16, 2, 0, 3],
        ).unwrap();
    
        let vertex_shader_src = r#"
            #version 140

            in vec2 position;
            uniform mat4 view;  // Uniform view matrix for aspect ratio control

            void main() {
                gl_Position = view * vec4(position, 0.0, 1.0);
            }
        "#;
    
        let fragment_shader_src = r#"
            #version 140
    
            out vec4 color;
    
            void main() {
                color = vec4(0.0, 0.0, 1.0, 1.0);
            }
        "#;
    
        let program = glium::Program::from_source(&self.display, vertex_shader_src, fragment_shader_src, None).unwrap();
    
        // Set the sprite_size uniform
        let uniforms = uniform! {
            view: *view_matrix.as_ref(),
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
                        glium::glutin::event::VirtualKeyCode::U => {Logger::info("Pressed U"); self.velocity[1] = 0.05},
                        glium::glutin::event::VirtualKeyCode::H => {Logger::info("Pressed H"); self.velocity[0] = -0.05},
                        glium::glutin::event::VirtualKeyCode::J => {Logger::info("Pressed J"); self.velocity[1] = -0.05},
                        glium::glutin::event::VirtualKeyCode::K => {Logger::info("Pressed K"); self.velocity[0] = 0.05},
                        _ => (),
                    }
                }
            }
            _ => (),
        }
    }
}