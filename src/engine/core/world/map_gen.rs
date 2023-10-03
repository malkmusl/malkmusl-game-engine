use std::fs::File;
use std::io::Write;
use std::io::Read;
use std::sync::Mutex;
use std::sync::Arc;
use ron::de::from_str;
use serde::Deserialize;

use crate::engine::assets_loader::texture_loader::TextureAtlas;
use crate::engine::assets_loader::texture_tilesets::*;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct TileData {
    position: [i32; 2],
    tileset: String,
    texture: i32,
    passable: Passable,
    encounter: i32,
    encounter_tables: Vec<i32>,
}

#[derive(Clone, Copy, Debug, serde::Serialize, serde::Deserialize)]
pub enum Passable {
    NORMAL,
    // Add other variants as needed
}

fn is_valid_coord(coord: u32) -> bool {
    coord % 2 != 0
}

pub fn generate_base_map_file(path: &str, x: u32, y: u32) {
    if !is_valid_coord(x) || !is_valid_coord(y) {
        println!("Invalid coordinates. Both x and y must be odd numbers.");
        return;
    }

    let tileset = "OUTSIDE_ATLAS";
    let texture = 1;
    let passable = Passable::NORMAL;
    let encounter = -1;
    let mut encounter_tables = Vec::new();

    let mut tiles = Vec::new();

    for i in -(x as i32 / 2)..=(x as i32 / 2) {
        for j in -(y as i32 / 2)..=(y as i32 / 2) {
            let position = [i, j];
            let tile_data = TileData {
                position,
                tileset: tileset.to_string(),
                texture,
                passable,
                encounter,
                encounter_tables: encounter_tables.clone(),
            };
            tiles.push(tile_data);
        }
    }

    let serialized = ron::ser::to_string_pretty(&tiles, Default::default()).expect("Serialization failed");

    let mut file = File::create(path).expect("Failed to create file");
    file.write_all(serialized.as_bytes()).expect("Failed to write to file");

    println!("Base map file generated successfully at: {}", path);
}

pub fn read_base_map_file(path: &str) -> Result<Vec<TileData>, Box<dyn std::error::Error>> {
    let mut file = File::open(path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;

    let tiles: Vec<TileData> = ron::de::from_str(&content)?;

    Ok(tiles)
}

fn get_texture_atlas(name: &str) -> Option<&'static Arc<Mutex<TextureAtlas>>> {
    match name {
        "BIKE_SHOP_INTERIOR_ATLAS" => Some(&BIKE_SHOP_INTERIOR_ATLAS),
        "BOAT_ATLAS" => Some(&BOAT_ATLAS),
        "CAVES_ATLAS" => Some(&CAVES_ATLAS),
        "DEPARTMENT_STORE_INTERIOR_ATLAS" => Some(&DEPARTMENT_STORE_INTERIOR_ATLAS),
        "DUNGEON_CAVE_ATLAS" => Some(&DUNGEON_CAVE_ATLAS),
        "DUNGEON_FOREST_ATLAS" => Some(&DUNGEON_FOREST_ATLAS),
        "FACTORY_INTERIOR_ATLAS" => Some(&FACTORY_INTERIOR_ATLAS),
        "GAME_CORNER_INTERIOR_ATLAS" => Some(&GAME_CORNER_INTERIOR_ATLAS),
        "GRAVEYARD_TOWER_INTERIOR_ATLAS" => Some(&GRAVEYARD_TOWER_INTERIOR_ATLAS),
        "GYMS_INTERIOR_ATLAS" => Some(&GYMS_INTERIOR_ATLAS),
        "HARBOUR_INTERIOR_ATLAS" => Some(&HARBOUR_INTERIOR_ATLAS),
        "INTERIOR_GENERAL_ATLAS" => Some(&INTERIOR_GENERAL_ATLAS),
        "MANSION_INTERIOR_ATLAS" => Some(&MANSION_INTERIOR_ATLAS),
        "MART_INTERIOR_ATLAS" => Some(&MART_INTERIOR_ATLAS),
        "MULTIPLAYER_ROOMS_ATLAS" => Some(&MULTIPLAYER_ROOMS_ATLAS),
        "MUSEUM_INTERIOR_ATLAS" => Some(&MUSEUM_INTERIOR_ATLAS),
        "POKE_CENTRE_INTERIOR_ATLAS" => Some(&POKE_CENTRE_INTERIOR_ATLAS),
        "RUINS_INTERIOR_ATLAS" => Some(&RUINS_INTERIOR_ATLAS),
        "TRAINER_TOWER_INTERIOR_ATLAS" => Some(&TRAINER_TOWER_INTERIOR_ATLAS),
        "UNDERGROUND_PATH_ATLAS" => Some(&UNDERGROUND_PATH_ATLAS),
        "UNDERWATER_ATLAS" => Some(&UNDERWATER_ATLAS),
        "OUTSIDE_ATLAS" => Some(&OUTSIDE_ATLAS),
        _ => None,
    }
}

fn convert_texture(){
    let temp_name = "OUTSIDE_ATLAS";
    let atlas = get_texture_atlas(temp_name);

    let texture_id = 2; //temp_id
    
    let texture = atlas.load_texture_from_map(31, self.display.clone());
}