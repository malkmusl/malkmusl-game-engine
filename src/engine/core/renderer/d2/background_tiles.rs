

use glium::implement_vertex;
use glium::texture::Texture2d;

#[derive(Copy, Clone)]
struct TileVertex {
    position: [f32; 2],
}

#[allow(dead_code)]
pub struct BackgroundTiles {
    display: glium::Display,
    tiles: Vec<Tile>,
}
#[allow(dead_code)]
struct Tile {
    position: [f32; 2],
    sprite_size: [f32; 2],
    sprite: Texture2d,
}

implement_vertex!(TileVertex, position);
