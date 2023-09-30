use crate::engine::assets_loader;
use crate::{engine::assets_loader::loader::ASSET_FOLDER, logger_error_assetloader};
use glium::texture::RawImage2d;
use image::{DynamicImage, GenericImageView};
use glium::{texture::SrgbTexture2d, Display};

use crate::engine::assets_loader::loader::ASSET_PREFIX;
use crate::engine::core::metadata::*;
use crate::engine::console_logger::logger::*;

pub struct TextureAtlas {
    pub atlas_image: DynamicImage,
    pub atlas_name: String,
    pub texture_size: [usize; 2],
    pub atlas_width: usize,
    pub atlas_height: usize,
}


impl TextureAtlas {
    pub fn new(path: &str, atlas_name: &str, texture_size: [usize; 2]) -> TextureAtlas {
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
            atlas_width: image_dimensions.0 as usize,
            atlas_height: image_dimensions.1 as usize,
        }
    }

    #[allow(dead_code)]
    fn get_rows(&self) -> usize {
        self.atlas_width / self.texture_size[0]
    }

    #[allow(dead_code)]
    fn get_columns(&self) -> usize {
        self.atlas_height / self.texture_size[1]
    }

    pub fn load_texture_from_atlas(&self, position: [usize; 2], display: Display) -> SrgbTexture2d {
        if position[0] == 0 || position[1] == 0 {
            println!("Failed to load Texture <{}>: the Texture Position can't be 0", self.atlas_name.to_uppercase());
            // You might want to return a default texture or handle this case appropriately
            assets_loader::loader::load_texture(&display.clone(), "apple.png");
        }

        let rows = self.get_rows();
        let cols = self.get_columns();

        let texture_position = position[0] + position[1] * rows - 1;

        if texture_position >= rows * cols {
            println!("Failed to load Texture <{}>: the Texture Position is out of range! Max Rows: {} Columns: {}", self.atlas_name.to_uppercase(), rows, cols);
            // You might want to return a default texture or handle this case appropriately
            assets_loader::loader::load_texture(&display.clone(), "apple.png");
        }

        let x = (texture_position % rows) * self.texture_size[0];
        let y = (texture_position / rows) * self.texture_size[1];

        println!("Loading texture at position ({}, {}) in atlas", x, y);

        let cropped_image = self.atlas_image.view(x as u32, y as u32, self.texture_size[0] as u32, self.texture_size[1] as u32);

        let texture_data = cropped_image.to_image();
        let image_dimensions = texture_data.dimensions();
        let raw_image = RawImage2d::from_raw_rgba_reversed(&texture_data.into_raw(), image_dimensions);

        println!("Creating SrgbTexture2d from raw image data");

        SrgbTexture2d::new(&display, raw_image).unwrap()
    }
}



pub fn load_texture_from_file(){

}
pub fn atlas_test(display: Display) {
    let outside_atlas = TextureAtlas::new("textures/tilesets/","outside", [32, 32]);
    let texture = outside_atlas.load_texture_from_atlas([1, 1], display);
    
}
