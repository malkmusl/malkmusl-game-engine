use glium::{Surface, implement_vertex, uniform};



#[derive(Clone)]
pub struct NPC {
    display: glium::Display,
    position: [f32; 2],
    velocity: [f32; 2],
    pub sprite_size: [f32; 2],
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
        self.set_velocity(0.0, 0.0);
        self.draw_sprite(frame);
    }

    pub fn draw_sprite(&mut self, frame: &mut glium::Frame) {
        let half_size = self.sprite_size[0] / 2.0;
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
                color = vec4(0.0, 0.0, 1.0, 1.0);
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
                        glium::glutin::event::VirtualKeyCode::U => {println!("Pressed U"); self.velocity[1] = 0.1},
                        glium::glutin::event::VirtualKeyCode::H => {println!("Pressed H"); self.velocity[0] = -0.1},
                        glium::glutin::event::VirtualKeyCode::J => {println!("Pressed J"); self.velocity[1] = -0.1},
                        glium::glutin::event::VirtualKeyCode::K => {println!("Pressed K"); self.velocity[0] = 0.1},
                        _ => (),
                    }
                }
            }
            _ => (),
        }
    }
}