use crate::engine::assets_loader;
use crate::engine::console_logger::logger;
use crate::engine::core::renderer::d2::background_tiles::Tile;
use crate::{engine::assets_loader::loader::ASSET_FOLDER, logger_error_assetloader};
use glium::texture::RawImage2d;
use glutin::display;
use image::{DynamicImage, GenericImageView, ImageBuffer, Rgba};
use glium::{texture::SrgbTexture2d, Display};
use std::collections::HashMap;
use std::sync::Arc;

use super::texture_tilesets::OUTSIDE_ATLAS;


pub struct TextureAtlas {
    pub atlas_image: DynamicImage,
    pub atlas_name: String,
    pub texture_size: [u32; 2],
    pub atlas_width: u32,
    pub atlas_height: u32,
    pub textures: HashMap<u32, ImageBuffer<Rgba<u8>, Vec<u8>>>, // Store texture data instead of SrgbTexture2d
}

pub struct TextureAtlasMap {
}


impl TextureAtlas {
    pub fn new(path: &str, atlas_name: &str, texture_size: [u32; 2]) -> TextureAtlas {
        let asset_path = format!("{}/{}{}.png", ASSET_FOLDER, path, atlas_name);
        println!("{}", asset_path);
        let image = image::open(&asset_path).map_err(|e| {
            println!("Error opening image: {:?}", e);
            
        }).expect("Failed to open Image");
        let image_dimensions = image.dimensions();
    
        println!("Loaded atlas image: {}x{}", image_dimensions.0, image_dimensions.1);
    
        TextureAtlas {
            atlas_image: image,
            atlas_name: atlas_name.to_string(),
            texture_size,
            atlas_width: image_dimensions.0,
            atlas_height: image_dimensions.1,
            textures: HashMap::new(),
        }
    }

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

    pub fn load_texture_from_tileset_to_map(&mut self, tileset_position: u32, display: Display) {
        if tileset_position == 0 {
            println!("Failed to load Texture <{}>: the Texture Position can't be 0", self.atlas_name.to_uppercase());
            // You might want to return a default texture or handle this case appropriately
            assets_loader::loader::load_texture(&display.clone(), "apple.png");
            return;
        }
    
        let rows = self.get_rows();
        let cols = self.get_columns();
    
        let mut id = 0;
    
        for col in 0..cols {
            for row in 0..rows {
                if id == tileset_position - 1 {
                    let x = col * self.texture_size[0];
                    let y = row * self.texture_size[1];
    
                    println!("Loading texture at position ({}, {}) in atlas", x, y);
    
                    let cropped_image = self.atlas_image.view(x as u32, y as u32, self.texture_size[0] as u32, self.texture_size[1] as u32);
                    let texture_data = cropped_image.to_image();
                    println!("Putting texture_data to textures");
                    self.textures.insert(id, texture_data);
    
                    return; // Exit the function once the texture is loaded
                }
    
                id += 1;
            }
        }
    
        // If the loop completes without finding the position, it's out of range
        println!("Failed to load Texture <{}>: the Texture Position is out of range! Max Rows: {} Columns: {}", self.atlas_name.to_uppercase(), rows, cols);
        // You might want to return a default texture or handle this case appropriately
        assets_loader::loader::load_texture(&display.clone(), "apple.png");
    }

    pub fn load_texture_from_map(&self, id: u32, display: Display) -> SrgbTexture2d {
        let err = logger::error_assets("Failed to load Texture with ");
        let ferr = format!("{} ID:{}", err, id);

        let texture_data = self.textures.get(&id).expect(&ferr);
        let image_dimensions = (self.texture_size[0] as u32, self.texture_size[1] as u32);
        let raw_image = RawImage2d::from_raw_rgba_reversed(&texture_data.clone().into_raw(), image_dimensions);

        SrgbTexture2d::new(&display, raw_image).unwrap()
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
    let texture = OUTSIDE_ATLAS.load_texture_from_map(id, display);

    let tile = Tile::new(pos, 0.1, texture);

}

pub fn atlas_test(display: Display) {
    let mut outside_atlas = &OUTSIDE_ATLAS;    
}
