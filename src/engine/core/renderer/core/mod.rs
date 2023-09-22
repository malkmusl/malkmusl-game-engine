pub mod opengl;
pub mod vulkano;

pub const GAME_STATE_DEBUG: bool = true;

#[derive(Copy, Clone, PartialEq)]
pub enum  GameStatus {
    Running,
    Paused,
    Stopped,
}
    