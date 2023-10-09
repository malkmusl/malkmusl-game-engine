extern crate lazy_static;

use super::background_tiles::Tile;
use crate::engine::assets_loader::loader::*;
use serde::Deserialize;
use std::fs;
use lazy_static::lazy_static;

lazy_static!{
    pub static ref MAP: Vec<TileData> = Vec::new();
}

#[derive(Deserialize)]
pub struct TileData {
    pub position: (i32, i32),
    pub sprite_size: f32,
    pub texture: i32,
    pub passable: PassableType, // You need to define PassableType enum
    pub encounter: i32,
    pub encounter_tables: Vec<i32>,
}

#[derive(Debug, Deserialize)]
pub enum PassableType {
    NORMAL,
    NOT,
    WATER,
    BIKE
}

pub fn load_tile_data() -> Result<Vec<TileData>, ron::Error> {
    let file_path = format!("{}/maps/twinleaf_town/layer_0.ron", ASSET_FOLDER);
    let contents = fs::read_to_string(file_path)?;
    let tile_data: Vec<TileData> = ron::de::from_str(&contents)?;

    Ok(tile_data)
}