extern crate lazy_static;
use lazy_static::lazy_static;

use std::fs;
use std::ffi::OsString;

use crate::engine::core::metadata::*;
use crate::engine::console_logger::logger::*;
use crate::{logger_info, logger_error};

pub const ASSET_FOLDER: &str = "./src/assets";

lazy_static! {
    pub static ref ASSET_PREFIX: String = {
        let prefix = "[AssetLoader]";
        set_color(COLOR_YELLOW, prefix)
    };
}

pub fn load_texture(){
    if let Ok(metadata) = fs::metadata(ASSET_FOLDER) {
        if metadata.is_dir() {
            logger_info!("{}{} The directory '{}' exists.", *ASSET_PREFIX, reset_color(), ASSET_FOLDER);
        } else {
            logger_error!("{}{} '{}' is not a directory.", *ASSET_PREFIX, reset_color(), ASSET_FOLDER);
        }
    } else {
        logger_error!("{}{} The directory '{}' does not exist.", *ASSET_PREFIX, reset_color(), ASSET_FOLDER);
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
                    logger_info!("{}{} {}", *ASSET_PREFIX, reset_color(),  log_message);
                } else {
                    eprintln!("Invalid UTF-8 sequence in file name");
                }
            }
        }
    } else {
        eprintln!("Error reading directory: {}", ASSET_FOLDER);
    }
}
