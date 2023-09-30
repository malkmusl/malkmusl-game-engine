
use chrono::Local;
use crate::engine::assets_loader::loader::ASSET_PREFIX;
use crate::engine::core::renderer::core::GAME_STATE_DEBUG;
use crate::engine::core::entity::player::Player;
use crate::engine::core::entity::player::{PLAYER_DEBUG, PLAYER_MOVEMENT_DEBUG};
use crate::engine::core::metadata::{DEBUG, COLOR_YELLOW, COLOR_CYAN};

use crate::engine::core::metadata;
use crate::engine::core::renderer::core::GameStatus;
use crate::engine::core::renderer::core::opengl::OPENGL_PREFIX;

/// Sets the color for a given text value and returns the formatted string.
/// 
/// # Arguments
///
/// * `color` - A `metadata::Color` struct representing the desired color.
/// * `value` - A reference to the text value that should be colored.
/// 
/// # Returns
///
/// A formatted string with ANSI escape codes for setting the text color.
/// 
/// # Example
/// 
/// ```
/// use metadata::Color;
/// let color = Color { red: 1.0, green: 0.0, blue: 0.0 };
/// let value = "Hello, World!";
/// let colored_text = set_color(color, value);
/// println!("{}", colored_text);
/// ```
pub fn set_color(color: metadata::Color, value: &str) -> String {
    let color = format!("\x1b[38;2;{};{};{}m{}", (color.red * 255.0) as u8, (color.green * 255.0) as u8, (color.blue * 255.0) as u8, value);
    return color;
}

/// Resets the text color and formatting to the default settings.
/// 
/// This function returns a string containing ANSI escape codes to reset the
/// text color and formatting to their default values.
/// 
/// # Returns
///
/// A formatted string with ANSI escape codes to reset the text color and formatting.
/// 
/// # Example
/// 
/// ```
/// let reset_code = reset_color();
/// println!("{}This text has been reset.", reset_code);
/// ```
pub fn reset_color() -> String {
    let color = format!("\x1b[0m");
    return color;
}

/// Gets the current local time and formats it as a string.
/// 
/// This function retrieves the current local time and formats it into a string
/// with the following format: "[YYYY-MM-DD][HH:MM:SS]". The time is based on
/// the system's local timezone.
/// 
/// # Returns
///
/// A formatted string representing the current local time.
/// 
/// # Example
/// 
/// ```
/// let current_time = get_time();
/// println!("Current time: {}", current_time);
/// ```
pub fn get_time() -> String {
    let time = Local::now().format("[%Y-%m-%d][%H:%M:%S]").to_string();
    return time;
}

/// Logs an informational message with a timestamp and green "[INFO]" prefix.
/// 
/// This function takes a message as input and logs it with an "[INFO]" prefix
/// in green color. It also includes a timestamp with dark grey color. The
/// logged message is printed to the console and returned as a formatted string.
/// 
/// # Arguments
/// 
/// * `args` - The informational message to be logged.
/// 
/// # Returns
/// 
/// A formatted string containing the log message.
/// 
/// # Example
/// 
/// ```
/// let message = "This is an information message.";
/// let log_result = info(message);
/// println!("{}", log_result);
/// ```
pub fn info(args: &str) -> String{
    let color =  metadata::COLOR_GREEN;
    let prefix = set_color(color, "[INFO] ");
    let time = set_color(metadata::COLOR_DARK_GREY, get_time().as_str())+ " ";
    let msg = reset_color() + &format!("{}", args);

    let log_message = format!("{}{}{}", time, prefix, msg);
    log_message
}

pub fn info_assets(args: &str) -> String{
    let color =  metadata::COLOR_GREEN;
    let prefix = set_color(color, "[INFO] ");
    let perfix_2 = set_color(COLOR_YELLOW, *&ASSET_PREFIX.as_str());
    let time = set_color(metadata::COLOR_DARK_GREY, get_time().as_str())+ " ";
    let msg = reset_color() + &format!(" {}", args);

    let log_message = format!("{}{}{}{}", time, prefix, perfix_2, msg);
    log_message
}

pub fn info_opengl(args: &str) -> String{
    let color =  metadata::COLOR_GREEN;
    let prefix = set_color(color, "[INFO] ");
    let perfix_2 = set_color(COLOR_CYAN, *&OPENGL_PREFIX.as_str());
    let time = set_color(metadata::COLOR_DARK_GREY, get_time().as_str())+ " ";
    let msg = reset_color() + &format!(" {}", args);

    let log_message = format!("{}{}{}{}", time, prefix, perfix_2, msg);
    log_message
}


/// Logs a warning message with a timestamp and yellow "[WARN]" prefix.
/// 
/// This function takes a message as input and logs it with a "[WARN]" prefix
/// in yellow color. It also includes a timestamp with dark grey color. The
/// logged message is printed to the console and returned as a formatted string.
/// 
/// # Arguments
/// 
/// * `args` - The warning message to be logged.
/// 
/// # Returns
/// 
/// A formatted string containing the log message.
/// 
/// # Example
/// 
/// ```
/// let message = "This is a warning message.";
/// let log_result = warn(message);
/// println!("{}", log_result);
/// ```
pub fn warn(args: &str) -> String{
    let color =  metadata::COLOR_YELLOW;
    let prefix = set_color(color, "[WARN]");
    let time = set_color(metadata::COLOR_DARK_GREY, get_time().as_str())+ " ";
    let msg = reset_color() + &format!(" {}", args);

    let log_message = format!("{}{}{}", time, prefix, msg);
    log_message
}

pub fn warn_assets(args: &str) -> String{
    let color =  metadata::COLOR_YELLOW;
    let prefix = set_color(color, "[WARN] ");
    let perfix_2 = set_color(COLOR_YELLOW, *&ASSET_PREFIX.as_str());
    let time = set_color(metadata::COLOR_DARK_GREY, get_time().as_str())+ " ";
    let msg = reset_color() + &format!("{}", args);

    let log_message = format!("{}{}{}{}", time, prefix, perfix_2, msg);
    log_message
}

pub fn warn_opengl(args: &str) -> String{
    let color =  metadata::COLOR_YELLOW;
    let prefix = set_color(color, "[WARN] ");
    let perfix_2 = set_color(COLOR_CYAN, *&OPENGL_PREFIX.as_str());
    let time = set_color(metadata::COLOR_DARK_GREY, get_time().as_str())+ " ";
    let msg = reset_color() + &format!(" {}", args);

    let log_message = format!("{}{}{}{}", time, prefix, perfix_2, msg);
    log_message
}
/// Logs an error message with a timestamp and red "[ERROR]" prefix.
/// 
/// This function takes a message as input and logs it with an "[ERROR]" prefix
/// in red color. It also includes a timestamp with dark grey color. The
/// logged message is printed to the console and returned as a formatted string.
/// 
/// # Arguments
/// 
/// * `args` - The error message to be logged.
/// 
/// # Returns
/// 
/// A formatted string containing the log message.
/// 
/// # Example
/// 
/// ```
/// let message = "This is an error message.";
/// let log_result = error(message);
/// println!("{}", log_result);
/// ```
pub fn error(args: &str) -> String{
    let color =  metadata::COLOR_RED;
    let prefix = set_color(color, "[ERROR] ");
    let time = set_color(metadata::COLOR_DARK_GREY, get_time().as_str())+ " ";
    let msg = reset_color() + &format!("{}", args);

    let log_message = format!("{}{}{}", time, prefix, msg);
    log_message
}

pub fn error_assets(args: &str) -> String{
    let color =  metadata::COLOR_RED;
    let prefix = set_color(color, "[ERROR] ");
    let perfix_2 = set_color(COLOR_YELLOW, *&ASSET_PREFIX.as_str());
    let time = set_color(metadata::COLOR_DARK_GREY, get_time().as_str())+ " ";
    let msg = reset_color() + &format!("{}", args);

    let log_message = format!("{}{}{}{}", time, prefix, perfix_2, msg);
    log_message
}

pub fn error_opengl(args: &str) -> String{
    let color =  metadata::COLOR_RED;
    let prefix = set_color(color, "[ERROR] ");
    let perfix_2 = set_color(COLOR_CYAN, *&OPENGL_PREFIX.as_str());
    let time = set_color(metadata::COLOR_DARK_GREY, get_time().as_str())+ " ";
    let msg = reset_color() + &format!("{}", args);

    let log_message = format!("{}{}{}{}", time, prefix, perfix_2, msg);
    log_message
}

/// Logs the game state with optional debugging information.
/// 
/// This function logs the current game state along with optional debugging information
/// if debugging flags are enabled (DEBUG or GAME_STATE_DEBUG). It supports different
/// game states such as "Running," "Paused," and "Stopped" with corresponding log prefixes
/// and messages. The logged message is printed to the console and returned as a formatted string.
/// 
/// # Arguments
/// 
/// * `state` - The current game state (GameStatus enum).
/// * `exit_code` - The exit code to log in case of "Stopped" state.
/// 
/// # Returns
/// 
/// A formatted string containing the log message, or an empty string if not in debugging mode.
/// 
/// # Example
/// 
/// ```
/// let game_status = GameStatus::Running;
/// let exit_code = 0;
/// let log_result = game_state(game_status, exit_code);
/// println!("{}", log_result);
/// ```
pub fn game_state(state: GameStatus, exit_code: i8) -> String {
    // Check if either DEBUG or GAME_STATE_DEBUG is enabled
    if DEBUG || GAME_STATE_DEBUG {
        // Match the game state to determine the log message
        match state {
            GameStatus::Running => {
                // Prepare the log message for "Running" state
                let message = set_color(metadata::COLOR_BLUE, "[GameState]") + " " + &reset_color() + "Game is running";
                // Log the message with a warning prefix
                let log_message = warn(message.as_str());
                // Return the logged message
                log_message
            },
            GameStatus::Paused => {
                // Prepare the log message for "Paused" state
                let message = set_color(metadata::COLOR_BLUE, "[GameState]") + " " + &reset_color() + "Game is paused";
                // Log the message with a warning prefix
                let log_message = warn(message.as_str());
                // Return the logged message
                log_message
            },
            GameStatus::Stopped => {
                // Prepare the log message for "Stopped" state
                let message = set_color(metadata::COLOR_BLUE, "[GameState]") + " " + &reset_color() + "Game is stopped... Exiting with code " + &exit_code.to_string() + "";
                // Log the message with an error prefix
                let log_message = error(message.as_str());
                // Return the logged message
                log_message
            },
        }
    } else {
        // Return an empty string if debugging is not enabled
        String::from("")
    }
}

/// Logs player movement with optional debugging information.
/// 
/// This function logs player movement along with optional debugging information
/// if debugging flags are enabled (DEBUG, PLAYER_DEBUG, or PLAYER_MOVEMENT_DEBUG).
/// It logs the player's name, keycode pressed, and current position. The logged message
/// is printed to the console and returned as a formatted string.
/// 
/// # Arguments
/// 
/// * `player` - A mutable reference to the player object.
/// * `keycode` - The keycode pressed by the player.
/// 
/// # Returns
/// 
/// A formatted string containing the log message, or an empty string if not in debugging mode.
/// 
/// # Example
/// 
/// ```
/// let mut player = Player::new("Alice");
/// let keycode = "W";
/// let log_result = debug_player_movement(&mut player, keycode);
/// println!("{}", log_result);
/// ```
pub fn debug_player_movement(player: &mut Player, keycode: &str) -> String {
    // Check if DEBUG or PLAYER_DEBUG or PLAYER_MOVEMENT_DEBUG is enabled
    if DEBUG || PLAYER_DEBUG || PLAYER_MOVEMENT_DEBUG {
        // Prepare the log message components
        let prefix = set_color(metadata::COLOR_MAGENTA, "[PlayerMovement]") + &reset_color();
        let name = set_color(metadata::COLOR_CYAN, format!("[{}]", player.name).as_str()) + &reset_color();
        
        // Create the log message
        let log = format!(
            "{} {} Pressed {} => X:{}/Y:{}",
            prefix,
            name,
            keycode,
            player.position[0],
            player.position[1]
        );

        // Log the message as an informational message
        info(log.as_str());

        // Return the logged message
        log
    } else {
        // Return an empty string if debugging is not enabled
        String::from("")
    }
}