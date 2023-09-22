use glium::Frame;
use rand::random;

use glium::Surface;
use glium::implement_vertex;
use glium::texture::{Texture2d, RawImage2d};

#[derive(Copy, Clone)]
struct TileVertex {
    position: [f32; 2],
}

pub struct BackgroundTiles {
    display: glium::Display,
    tiles: Vec<Tile>,
}

struct Tile {
    position: [f32; 2],
    sprite_size: [f32; 2],
    sprite: Texture2d,
}

implement_vertex!(TileVertex, position);
