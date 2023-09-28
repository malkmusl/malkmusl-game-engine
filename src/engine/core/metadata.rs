extern crate lazy_static;
use lazy_static::lazy_static;
use std::fs::File;
use std::io::Read;


// crate const variables
pub const ENGINE_NAME: &str = "malkmusl Rust Game Engine";
pub const VSYNC: bool = false;
pub const DEBUG: bool = false;

// crate const colors
pub struct Color {
    pub red: f32,
    pub green: f32,
    pub blue: f32,
    pub alpha: f32,
}

pub const COLOR_RED: Color = Color { red: 1.0, green: 0.0, blue: 0.0, alpha: 1.0 };
pub const COLOR_GREEN: Color = Color { red: 0.0, green: 1.0, blue: 0.0, alpha: 1.0 };
pub const COLOR_YELLOW: Color = Color { red: 1.0, green: 1.0, blue: 0.0, alpha: 1.0 };
pub const COLOR_BLUE: Color = Color { red: 0.0, green: 0.0, blue: 1.0, alpha: 1.0 };
pub const COLOR_MAGENTA: Color = Color { red: 1.0, green: 0.0, blue: 1.0, alpha: 1.0 };
pub const COLOR_CYAN: Color = Color { red: 0.0, green: 1.0, blue: 1.0, alpha: 1.0 };
#[allow(dead_code)]
pub const COLOR_WHITE: Color = Color { red: 1.0, green: 1.0, blue: 1.0, alpha: 1.0 };
#[allow(dead_code)]
pub const COLOR_BLACK: Color = Color { red: 0.0, green: 0.0, blue: 0.0, alpha: 1.0 };
pub const COLOR_DARK_GREY: Color = Color { red: 0.25, green: 0.25, blue: 0.25, alpha: 1.0 };


// Declare a static variable for the engine version
lazy_static! {
    pub static ref ENGINE_VERSION: String = {
        // Use the get_cargo_toml_version function to read the version
        match get_cargo_toml_version() {
            Some(version) => version,
            None => "Version not found".to_string(),
        }
    };
}


fn get_cargo_toml_version() -> Option<String> {
    // Open the Cargo.toml file
    let mut toml_file = File::open("Cargo.toml").ok()?;

    // Read the contents of the Cargo.toml file into a string
    let mut toml_contents = String::new();
    toml_file.read_to_string(&mut toml_contents).ok()?;

    // Parse the TOML contents into a TOML table
    let toml_table: toml::Value = toml::from_str(&toml_contents).ok()?;

    // Extract the version string from the TOML table
    let version = toml_table
        .as_table()?
        .get("package")?
        .get("version")?
        .as_str()?
        .to_owned();

    Some(version)
}