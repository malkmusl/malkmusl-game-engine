pub mod opengl;
pub mod vulkano;

#[derive(Copy, Clone, PartialEq)]
pub enum  GameStatus {
    Running,
    Paused,
    Stopped,
}
    