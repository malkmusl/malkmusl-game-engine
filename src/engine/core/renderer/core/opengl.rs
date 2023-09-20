extern crate glium;

use glium::Surface;

use crate::engine::core::metadata::{ENGINE_NAME, ENGINE_VERSION, VSYNC};


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
    let display = glium::Display::new(wb, cb, &events_loop).unwrap();



    // 5. start EventsLoop
    events_loop.run(move |event, _, control_flow| {
        // 6. Handle events here
        match event {
            glium::glutin::event::Event::WindowEvent { event, .. } => match event {
                glium::glutin::event::WindowEvent::CloseRequested => {
                    *control_flow = glium::glutin::event_loop::ControlFlow::Exit
                }
                _ => (),
            },
            glium::glutin::event::Event::MainEventsCleared => {
                // Redraw the application.
                // 7. Redraw code here
                let mut frame = display.draw();
                frame.clear_color(1.0, 0.0, 0.0, 1.0);
                frame.finish().unwrap();
                
                // 8. Redraw code ends here
            },
            //close on escape
            glium::glutin::event::Event::DeviceEvent { event, .. } => match event {
                glium::glutin::event::DeviceEvent::Key(input) => match input.virtual_keycode {
                    Some(glium::glutin::event::VirtualKeyCode::Escape) => {
                        *control_flow = glium::glutin::event_loop::ControlFlow::Exit
                    }
                    _ => (),
                },
                _ => (),
            },
            _ => (),
        }
    }); 
}
