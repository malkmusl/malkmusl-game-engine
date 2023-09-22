
use chrono::Local;

use crate::engine::core::metadata;
use crate::engine::core::renderer::core::GameStatus;


pub fn set_color(color: metadata::Color, value: &str) -> String {
    let color = format!("\x1b[38;2;{};{};{}m{}", (color.red * 255.0) as u8, (color.green * 255.0) as u8, (color.blue * 255.0) as u8, value);
    return color;
}

pub fn reset_color() -> String {
    let color = format!("\x1b[0m");
    return color;
}

pub fn get_time() -> String {
    let time = Local::now().format("[%Y-%m-%d][%H:%M:%S]").to_string();
    return time;
}

pub fn info(args: &str) -> String{
    let color =  metadata::COLOR_GREEN;
    let prefix = set_color(color, "[INFO] ");
    let time = set_color(metadata::COLOR_DARK_GREY, get_time().as_str())+ " ";
    let msg = reset_color() + &format!("{}", args);

    let log_message = format!("{}{}{}", time, prefix, msg);
    println!("{}", log_message.as_str());
    log_message
}

pub fn warn(args: &str) -> String{
    let color =  metadata::COLOR_YELLOW;
    let prefix = set_color(color, "[WARN] ");
    let time = set_color(metadata::COLOR_DARK_GREY, get_time().as_str())+ " ";
    let msg = reset_color() + &format!("{}", args);

    let log_message = format!("{}{}{}", time, prefix, msg);
    println!("{}", log_message.as_str());
    log_message
}

pub fn error(args: &str) -> String{
    let color =  metadata::COLOR_RED;
    let prefix = set_color(color, "[ERROR] ");
    let time = set_color(metadata::COLOR_DARK_GREY, get_time().as_str())+ " ";
    let msg = reset_color() + &format!("{}", args);

    let log_message = format!("{}{}{}", time, prefix, msg);
    println!("{}", log_message.as_str());
    log_message
}

pub fn game_state(state: GameStatus, exit_code: i8) -> String{
    match state {
        GameStatus::Running => {
            let message = set_color(metadata::COLOR_BLUE, "[GameState]") + " " + &reset_color() + "Game is running";
            let log_message = warn(message.as_str());
            log_message
        },
        GameStatus::Paused => {
            let message = set_color(metadata::COLOR_BLUE, "[GameState]") + " " + &reset_color() + "Game is paused";
            let log_message = warn(message.as_str());
            log_message
        },
        GameStatus::Stopped => {
            let message = set_color(metadata::COLOR_BLUE, "[GameState]") + " " + &reset_color() + "Game is stopped... Exiting with code " + &exit_code.to_string() + "";
            let log_message = error(message.as_str());
            log_message
        },
    }
}