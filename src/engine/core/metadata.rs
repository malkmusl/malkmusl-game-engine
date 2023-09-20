// crate const variables
pub const ENGINE_NAME: &str = "malkmusl Rust Game Engine";
pub const ENGINE_VERSION: &str = "0.1.0";
pub const VSYNC: bool = true;

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
pub const COLOR_WHITE: Color = Color { red: 1.0, green: 1.0, blue: 1.0, alpha: 1.0 };
pub const COLOR_BLACK: Color = Color { red: 0.0, green: 0.0, blue: 0.0, alpha: 1.0 };
pub const COLOR_DARK_GREY: Color = Color { red: 0.25, green: 0.25, blue: 0.25, alpha: 1.0 };



