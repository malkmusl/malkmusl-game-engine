#![allow(dead_code)]
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct Sprites {
    normal: SpriteSet,
    shiny: SpriteSet,
}

#[derive(Clone, Debug, Deserialize)]
pub struct SpriteSet {
    front: String,
    back: String,
}

impl Sprites {
    pub fn new(
        front_sprite: String,
        back_sprite: String,
        front_shiny_sprite: String,
        back_shiny_sprite: String,
    ) -> Sprites {
        Sprites { 
            normal: SpriteSet::new(front_sprite, back_sprite), 
            shiny: SpriteSet::new(front_shiny_sprite, back_shiny_sprite) 
        }
    }
}

impl SpriteSet {
    pub fn new(
        front_sprite: String,
        back_sprite: String,
    ) -> SpriteSet{
        SpriteSet { 
            front: front_sprite, 
            back: back_sprite 
        }
    }
}