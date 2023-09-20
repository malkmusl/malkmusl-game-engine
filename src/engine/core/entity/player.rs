use glium::{Surface, implement_vertex, uniform};


use crate::engine::core::{*, consoleLogger::Logger};
pub struct Player {
    display: glium::Display,
    position: [f32; 2],
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
    pub fn new(display: glium::Display) -> Player {
        Player {
            display: display,
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

    pub fn update_player(&mut self) {
        self.position[0] += self.velocity[0];
        self.position[1] += self.velocity[1];
        self.sprite_size = [0.5, 0.5];
        self.draw_sprite();
    }

    pub fn draw_sprite(&mut self) {
        let half_size = self.sprite_size[0] / 2.0;
    
        let vertex_buffer = glium::VertexBuffer::new(&self.display, &[
            PlayerSprite { position: [-half_size + self.position[0], -half_size + self.position[1]] },
            PlayerSprite { position: [half_size + self.position[0], -half_size + self.position[1]] },
            PlayerSprite { position: [half_size + self.position[0], half_size + self.position[1]] },
            PlayerSprite { position: [-half_size + self.position[0], half_size + self.position[1]] },
        ]).expect(&Logger::error("Failed to create vertex buffer."));
    
        let index_buffer = glium::IndexBuffer::new(
            &self.display,
            glium::index::PrimitiveType::TriangleStrip,
            &[1 as u16, 2, 0, 3],
        ).expect(&Logger::error("Failed to create index buffer."));
    
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
                color = vec4(1.0, 0.0, 0.0, 1.0);
            }
        "#;
    
        let program = glium::Program::from_source(&self.display, vertex_shader_src, fragment_shader_src, None).expect(&Logger::error("Failed to create shader program."));
    
        // Set the sprite_size uniform
        let uniforms = uniform! {
            spriteSize: self.sprite_size,
        };

        let mut target = self.display.draw();
        target.clear_color(0.0, 0.0, 0.0, 0.0);
        target.draw(
            &vertex_buffer,
            &index_buffer,
            &program,
            &uniforms,
            &Default::default(),
        ).expect(&Logger::error("Failed to draw."));
        target.finish().expect(&Logger::error("Failed to finish drawing."));
    }
    
}


pub fn handle_input(event: glium::glutin::event::WindowEvent, player: &mut Player) {
    match event {
        glium::glutin::event::WindowEvent::KeyboardInput { input, .. } => {
            if let Some(keycode) = input.virtual_keycode {
                match keycode {
                    glium::glutin::event::VirtualKeyCode::W => {println!("Pressed W"); player.velocity[1] = 0.1},
                    glium::glutin::event::VirtualKeyCode::A => {println!("Pressed A"); player.velocity[0] = -0.1},
                    glium::glutin::event::VirtualKeyCode::S => {println!("Pressed S"); player.velocity[1] = -0.1},
                    glium::glutin::event::VirtualKeyCode::D => {println!("Pressed D"); player.velocity[0] = 0.1},
                    _ => {player.velocity[0] = 0.0; player.velocity[0] = 0.0},
                }
            }
        }
        _ => (),
    }
}



