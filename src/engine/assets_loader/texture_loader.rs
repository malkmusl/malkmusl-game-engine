#![allow(dead_code, unused_variables)]
use crate::{logger_warn_assetloader, logger_error_assetloader, logger_info_assetloader};
use crate::engine::assets_loader::loader::*;
use crate::engine::assets_loader;
use crate::engine::console_logger::logger;
use crate::engine::core::renderer::d2::background_tiles::Tile;
use glium::texture::RawImage2d;
use image::{DynamicImage, GenericImageView, ImageBuffer, Rgba};
use glium::{texture::SrgbTexture2d, Display};
use std::collections::HashMap;

use crate::engine::core::metadata::*;
use crate::engine::console_logger::logger::*;

use super::texture_tilesets::OUTSIDE_ATLAS;


#[derive(Clone)]
pub struct TextureAtlas {
    pub atlas_image: DynamicImage,
    pub atlas_name: String,
    pub texture_size: [u32; 2],
    pub atlas_width: u32,
    pub atlas_height: u32,
    pub textures: HashMap<u32, ImageBuffer<Rgba<u8>, Vec<u8>>>, // Store texture data instead of SrgbTexture2d
}


impl TextureAtlas {
    pub fn new(path: &str, atlas_name: &str, texture_size: [u32; 2]) -> TextureAtlas {
        let asset_path = format!("{}/{}{}.png", ASSET_FOLDER, path, atlas_name);
        logger_info_assetloader!("Loading tileset from {}", asset_path);
        let image = image::open(&asset_path).map_err(|e| {
            logger_error_assetloader!("Error opening image: {:?}", e);
            
        }).expect("Failed to open Image");
        let image_dimensions = image.dimensions();
    
        logger_info_assetloader!("Loaded atlas image: {}x{}", image_dimensions.0, image_dimensions.1);
    
        TextureAtlas {
            atlas_image: image,
            atlas_name: atlas_name.to_string(),
            texture_size,
            atlas_width: image_dimensions.0,
            atlas_height: image_dimensions.1,
            textures: HashMap::new(),
        }
    }

    #[allow(dead_code)]
    pub fn get_name(self) -> String{
        self.atlas_name
    }

    #[allow(dead_code)]
    fn get_rows(&self) -> u32 {
        self.atlas_width / self.texture_size[0]
    }

    #[allow(dead_code)]
    fn get_columns(&self) -> u32 {
        self.atlas_height / self.texture_size[1]
    }

    pub fn load_texture_from_tileset_to_map(&mut self, display: &Display) {
    
        let rows = self.get_rows();
        if rows <= 0 {logger_error_assetloader!("Failed to get Rows form image!");} 

        let cols = self.get_columns();
        if rows <= 0 {logger_error_assetloader!("Failed to get Collums form image!");} 

        let max = rows * cols;

        let mut id: u32 = 0;
    
        for col in 0..cols {
            for row in 0..rows {
                if id < max {
                    let texture_width = self.texture_size[0] as u32;
                    let texture_height = self.texture_size[1] as u32;

                    let y = col * texture_width;
                    let x = row * texture_height;

                    logger_warn_assetloader!("Loading texture [{}] at position ({}, {}) in atlas", id, x, y);

                    let cropped_image = self.atlas_image.view(x, y, texture_width, texture_height);
        
        
                    let texture_data = cropped_image.to_image();
                    self.textures.insert(id, texture_data);
        
                    id += 1;
                } else {
                    // Exit the loop when id reaches max
                    return;
                }
            }
        }
         
        // If the loop completes without finding the position, it's out of range
        if id > max{logger_error_assetloader!("Failed to load Texture <{}>: the Texture Position is out of range! Max Rows: {} Columns: {}", self.atlas_name.to_uppercase(), rows, cols);}
        // You might want to return a default texture or handle this case appropriately
        assets_loader::loader::load_texture(&display.clone(), "apple.png");
    }

    pub fn load_texture_from_map(&self, id: u32, display: Display) -> SrgbTexture2d {
        let err = logger::error_assets("Failed to load Texture with ");
        let ferr = format!("{} ID:{} ", err, id);

        let texture_data = match self.textures.get(&id) {
            Some(data) => data,
            None => {
                //logger_error_assetloader!("Texture with ID: {} not found", id);
                // You can choose how to handle this case, e.g., returning a default texture.
                return SrgbTexture2d::empty(&display, 1, 1).unwrap();
            }
        };

        let image_dimensions = (32 as u32, 32 as u32);
        let raw_image = RawImage2d::from_raw_rgba_reversed(&texture_data.clone().into_raw(), image_dimensions);

        match SrgbTexture2d::new(&display, raw_image) {
            Ok(texture) => texture,
            Err(e) => {
                eprintln!("Failed to create texture: {}", e);
                // You can choose how to handle this case, e.g., returning a default texture.
                SrgbTexture2d::empty(&display, 1, 1).unwrap()
            }
        }

    }
    
}


pub fn load_tiles_from_file(map_name: &str, display: Display){
    //load map file 
    //load every Tile and pass the tileset_name tileset possition and the world positions
    
    //load &str tileset from file and looks if a tileset with name exist -> getTileset()
    let tileset ="outside";
    let pos = [0.0, 0.0];
    //loads tileset_pos and world_pos and pass it to 
    let id = 0;
    // example 

    //let tile = Tile::new(pos, 0.1, texture);

}

pub fn atlas_test(display: Display) {
    let outside_atlas = &OUTSIDE_ATLAS;    
}
