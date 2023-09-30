use crate::{engine::assets_loader::loader::ASSET_FOLDER, logger_error_assetloader};
use image::{DynamicImage, GenericImageView};
use glium::{texture::SrgbTexture2d, Display};

use crate::engine::assets_loader::loader::ASSET_PREFIX;
use crate::engine::core::metadata::*;
use crate::engine::console_logger::logger::*;

pub struct TextureAtlas {
    pub display: Display,
    pub atlas_image: DynamicImage,
    pub atlas_name: String,
    pub texture_size: [usize; 2],
    pub atlas_width: usize,
    pub atlas_height: usize,
}

impl TextureAtlas {
    pub fn new(atlas_name: &str, texture_size: [usize; 2], display: Display) -> Result<TextureAtlas, String> {
        let asset_path = format!("{}/{}.png", ASSET_FOLDER, atlas_name);
        let image = image::open(&asset_path).map_err(|e| format!("Failed to open image: {}", e))?;
        let image_dimensions = image.dimensions();

        Ok(TextureAtlas {
            display: display.clone(),
            atlas_image: image,
            atlas_name: atlas_name.to_string(),
            texture_size,
            atlas_width: image_dimensions.0 as usize,
            atlas_height: image_dimensions.1 as usize,
        })
    }

    #[allow(dead_code)]
    fn get_rows(&self) -> usize {
        self.atlas_width / self.texture_size[0]
    }

    #[allow(dead_code)]
    fn get_columns(&self) -> usize {
        self.atlas_height / self.texture_size[1]
    }

    pub fn load_texture_from_atlas(&self, position: [usize; 2]) -> SrgbTexture2d {
        if position[0] == 0 || position[1] == 0 {
            logger_error_assetloader!(
                "Failed to load Texture <{}>: the Texture Position can't be 0",
                self.atlas_name.to_uppercase()
            );
            // You might want to return a default texture or handle this case appropriately
            unimplemented!();
        }

        let texture_position = position[0] * position[1] - 1;

        if texture_position >= self.get_rows() * self.get_columns() - 1 {
            logger_error_assetloader!(
                "Failed to load Texture <{}>: the Texture Position is out of range! Max Rows: {} Columns: {}",
                self.atlas_name.to_uppercase(),
                self.get_rows(),
                self.get_columns()
            );
            // You might want to return a default texture or handle this case appropriately
            unimplemented!();
        }

        let x = (texture_position % self.get_rows()) * self.texture_size[0];
        let y = (texture_position / self.get_rows()) * self.texture_size[1];

        let cropped_image = self.atlas_image.view(x as u32, y as u32, self.texture_size[0] as u32, self.texture_size[1] as u32);

        let texture_data = cropped_image.to_image();
        let image_dimensions = texture_data.dimensions();
        let raw_image = glium::texture::RawImage2d::from_raw_rgba_reversed(&texture_data.into_raw(), image_dimensions);

        let texture = glium::texture::SrgbTexture2d::new(&self.display, raw_image).unwrap();
        texture
    }
}


pub fn load_texture_from_file(){
    
}
pub fn atlas_test(display: Display) {
    if let Ok(outside_atlas) = TextureAtlas::new("outside", [32, 32], display.clone()) {
        let texture = outside_atlas.load_texture_from_atlas([1, 1]);
    }
}
