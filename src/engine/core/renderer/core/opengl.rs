extern crate glium;

use glium::Frame;
use crate::engine::console_logger::Logger;
use crate::engine::core::entity::npc;
use crate::engine::core::metadata::{ENGINE_NAME, ENGINE_VERSION, VSYNC};
use crate::engine::core::entity::player;
use crate::engine::core::renderer::D2::testing;

use super::GameStatus;


#[allow(unused_mut)]
pub fn create_opengl_window(game_name: &str, game_width: f64, game_height: f64) {
    let graphics_api = "OpenGL";
    let mut state = GameStatus::Running;

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


    let mut player = player::Player::new(display.clone(), "makmusl".to_string());
    let mut npc = npc::NPC::new(display.clone());
    
    // 5. start EventsLoop
    events_loop.run(move |event, _, control_flow| {
        // 6. Handle events here
        match event {
            glium::glutin::event::Event::WindowEvent { mut event, .. } => {
                match event {
                    glium::glutin::event::WindowEvent::CloseRequested => {
                        *control_flow = glium::glutin::event_loop::ControlFlow::Exit;
                        state = GameStatus::Stopped;
                        Logger::game_state(state, 0);
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
                                        Logger::game_state(state, 21);
                                    }else{
                                        state = GameStatus::Running;
                                        Logger::game_state(state, 22);
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
fn update_content(display: glium::Display,player: &mut player::Player, npc: &mut npc::NPC){
    let mut frame = display.draw();
    //draw_squareV2(display.clone(), &mut frame);
    update_background_tiles(display.clone(), &mut frame);
    update_player(player, &mut frame);
    update_npc(npc, &mut frame);
    frame.finish().unwrap();
}



fn update_player(player: &mut player::Player, frame: &mut Frame) {
    player.update(frame);
}

fn update_npc(npc: &mut npc::NPC, mut frame: &mut Frame) {
    npc.update(&mut frame);
}

fn update_background_tiles(display: glium::Display, frame: &mut Frame){
    //background_tiles::draw(display.clone(), frame, 10, 10, 0.5);
    testing::simple_square::draw_square_v2(display.clone(), frame);

}