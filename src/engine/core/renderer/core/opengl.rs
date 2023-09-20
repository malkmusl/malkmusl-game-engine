extern crate glium;

use glium::Surface;
use glium::implement_vertex;

use crate::engine::core::consoleLogger::Logger;
use crate::engine::core::metadata::{ENGINE_NAME, ENGINE_VERSION, VSYNC};
use crate::engine::core::entity::player;


#[allow(unused_mut)]
pub fn create_opengl_window(game_name: &str, game_width: f64, game_height: f64) {
    let graphics_api = "OpenGL";

    let app_name = game_name.to_owned() + " - [" + ENGINE_NAME + " v"+ENGINE_VERSION+ " - "+ graphics_api+"]"; 


    // 1. The **winit::EventsLoop** for handling events.
    let mut events_loop = glium::glutin::event_loop::EventLoop::new();
    // 2. Parameters for building the Window.
    let wb = glium::glutin::window::WindowBuilder::new()
        .with_inner_size(glium::glutin::dpi::LogicalSize::new(game_width, game_height))
        .with_title(app_name);
    // 3. Parameters for building the OpenGL context.
    let cb = glium::glutin::ContextBuilder::new().with_vsync(VSYNC);
    // 4. Build the Display with the given window and OpenGL context parameters and register the
    //    window with the events_loop.
    let display = glium::Display::new(wb, cb, &events_loop).expect(&Logger::error("Failed to create display."));
    let mut frame = display.draw();
    let mut player = player::Player::new(display.clone());
    player.sprite_size = [0.5, 0.5];
    frame.clear_color(0.0, 0.0, 0.0, 1.0);
    frame.finish().expect(&Logger::error("Failed to finish frame."));

    // 5. start EventsLoop
    events_loop.run(move |event, _, control_flow| {
        // 6. Handle events here
        match event {
            glium::glutin::event::Event::WindowEvent { event, .. } => {
                match event {
                    glium::glutin::event::WindowEvent::CloseRequested => {
                        *control_flow = glium::glutin::event_loop::ControlFlow::Exit;
                    }
                    _ => {
                        player.draw_sprite();
                        player::handle_input(event, &mut player);
                    }
                }
            },

            glium::glutin::event::Event::MainEventsCleared => {
                player.update_player();
                player.set_velocity(0.0, 0.0);
            },
    
            glium::glutin::event::Event::DeviceEvent { event, .. } => {
                match event {
                    glium::glutin::event::DeviceEvent::Key(input) => {
                        match input.virtual_keycode {
                            Some(glium::glutin::event::VirtualKeyCode::Escape) => {
                                *control_flow = glium::glutin::event_loop::ControlFlow::Exit
                            }
                            _ => (),
                        }
                    }
                    _ => (),
                }
            },
            
            _ => (),
        }
        
    });
}

#[derive(Copy, Clone)]
struct MyVertex {
    position: [f32; 2],
}

// you must pass the list of members to the macro
implement_vertex!(MyVertex, position);

fn draw_square(display: glium::Display) {
    let vertex_buffer = glium::VertexBuffer::new(&display, &[
        MyVertex { position: [-0.5, -0.5] },
        MyVertex { position: [0.5, -0.5] },
        MyVertex { position: [0.5, 0.5] },
        MyVertex { position: [-0.5, 0.5] },
    ]).unwrap();

    let index_buffer = glium::IndexBuffer::new(
        &display,
        glium::index::PrimitiveType::TriangleStrip,
        &[1 as u16, 2, 0, 3],
    ).unwrap();

    let vertex_shader_src = r#"
        #version 140

        in vec2 position;

        void main() {
            gl_Position = vec4(position, 0.0, 1.0);
        }
    "#;

    let fragment_shader_src = r#"
        #version 140

        out vec4 color;

        void main() {
            color = vec4(1.0, 0.0, 0.0, 1.0);
        }
    "#;

    let program = glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap();

    let mut target = display.draw();
    target.clear_color(0.0, 0.0, 0.0, 0.0);
    target.draw(
        &vertex_buffer,
        &index_buffer,
        &program,
        &glium::uniforms::EmptyUniforms,
        &Default::default(),
    ).unwrap();
    target.finish().unwrap();
} 

