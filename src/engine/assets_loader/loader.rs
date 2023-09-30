extern crate lazy_static;
use glium::Display;
use lazy_static::lazy_static;

use ron::de::from_str;
use ron::*;
use std::io::Read;
use serde::Deserialize;

use std::fs;
use std::fs::File;
use std::ffi::OsString;

use crate::engine::core::metadata::*;
use crate::engine::console_logger::logger::{*, self};
use crate::{logger_info_assetloader, logger_error_assetloader, logger_warn_assetloader};

pub const ASSET_FOLDER: &str = "./src/assets";

lazy_static! {
    pub static ref ASSET_PREFIX: String = {
        let prefix = "[AssetLoader]";
        set_color(COLOR_YELLOW, prefix)
    };
}

pub fn dir_exist(){
    if let Ok(metadata) = fs::metadata(ASSET_FOLDER) {
        if metadata.is_dir() {
            logger_info_assetloader!("The directory '{}' exists.", ASSET_FOLDER);
        } else {
            logger_warn_assetloader!("'{}' is not a directory.", ASSET_FOLDER);
        }
    } else {
        logger_error_assetloader!("The directory '{}' does not exist.", ASSET_FOLDER);
        let _ = fs::create_dir_all(ASSET_FOLDER);
    }   
}


pub fn list_files() {
    if let Ok(entries) = fs::read_dir(ASSET_FOLDER) {
        for entry in entries {
            if let Ok(entry) = entry {
                let file_name: OsString = entry.file_name();
                if let Some(file_name_str) = file_name.to_str() {
                    let log_message = format!("Found file: {}", file_name_str);
                    logger_info_assetloader!("{}", log_message);
                } else {
                    logger_error_assetloader!("Invalid UTF-8 sequence in file name");
                }
            }
        }
    } else {
        logger_error_assetloader!("Error reading directory: {}", ASSET_FOLDER);
    }
}

pub fn load_texture(display: &Display, texture_name: &str) -> glium::texture::SrgbTexture2d {
    let asset_path = format!("{}/{}", ASSET_FOLDER, texture_name);
    let file_contents = std::fs::read(&asset_path).expect(&logger::error("Failed to read file"));
    let image = image::load(std::io::Cursor::new(&file_contents), image::ImageFormat::Png)
        .unwrap()
        .to_rgba8();
    let image_dimensions = image.dimensions();
    let image = glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);
    let texture = glium::texture::SrgbTexture2d::new(display, image).unwrap();
    texture
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct TileData {
    position: [f32; 2],
    sprite_size: f32,
    texture_name: String,
}

pub fn load_tiles_from_file(map: &str) -> Result<Vec<TileData>, Box<dyn std::error::Error>> {
    let file_path = format!("{}/maps/{}.ron", ASSET_FOLDER, map);
    // Open the file
    let mut file = File::open(file_path)?;
    
    // Read the file content into a string
    let mut content = String::new();
    file.read_to_string(&mut content)?;

    // Deserialize the RON data into a Vec<TileData>
    let tiles: Vec<TileData> = from_str(&content)?;
    Ok(tiles)
}
