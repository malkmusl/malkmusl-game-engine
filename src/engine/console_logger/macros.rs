
//default message
#[macro_export]
macro_rules! logger_message {
    ($color:expr, $prefix:expr, $($args:tt)*) => {
        let prefix = set_color(color, "[WARN] ");
        let time = set_color(metadata::COLOR_DARK_GREY, get_time().as_str())+ " ";
        let msg = reset_color() +format_args!($($args)*).to_string().as_str();

        let log_message = format!("{}{}{}", time, prefix, msg);
        println!("{}", log_message.as_str());
    };
}

//general Logger Messages
#[macro_export]
macro_rules! logger_info {
    ($($args:tt)*) => {
        let color = COLOR_GREEN;
        let prefix = set_color(color, "[INFO] ");
        let time = set_color(COLOR_DARK_GREY, get_time().as_str())+ " ";
        let msg = reset_color() +format_args!($($args)*).to_string().as_str();

        let log_message = format!("{}{}{}", time, prefix, msg);
        println!("{}", log_message.as_str());
    };
}

#[macro_export]
macro_rules! logger_warn {
    ($($args:tt)*) => {
        let color = COLOR_YELLOW;
        let prefix = set_color(color, "[WARN] ");
        let time = set_color(COLOR_DARK_GREY, get_time().as_str())+ " ";
        let msg = reset_color() +format_args!($($args)*).to_string().as_str();

        let log_message = format!("{}{}{}", time, prefix, msg);
        println!("{}", log_message.as_str());
    };
}

#[macro_export]
macro_rules! logger_error {
    ($($args:tt)*) => {
        let color =  COLOR_RED;
        let prefix = set_color(color, "[ERROR] ");
        let time = set_color(COLOR_DARK_GREY, get_time().as_str())+ " ";
        let msg = reset_color() +format_args!($($args)*).to_string().as_str();

        let log_message = format!("{}{}{}", time, prefix, msg);
        println!("{}", log_message.as_str());
    };
}

//AssetLoader messages
#[macro_export]
macro_rules! logger_info_assetloader {
    ($($args:tt)*) => {
        let color = COLOR_GREEN;
        let prefix = set_color(color, "[INFO] ");
        let perfix_2 = format!("{} ", set_color(COLOR_YELLOW, *&ASSET_PREFIX.as_str()));
        let time = set_color(COLOR_DARK_GREY, get_time().as_str())+ " ";
        let msg = reset_color() +format_args!($($args)*).to_string().as_str();

        let log_message = format!("{}{}{} {}", time, prefix, perfix_2, msg);
        println!("{}", log_message.as_str());
    };
}

#[macro_export]
macro_rules! logger_warn_assetloader {
    ($($args:tt)*) => {
        let color = COLOR_YELLOW;
        let prefix = set_color(color, "[WARN] ");
        let perfix_2 = format!("{} ", set_color(COLOR_YELLOW, *&ASSET_PREFIX.as_str()));
        let time = set_color(COLOR_DARK_GREY, get_time().as_str())+ " ";
        let msg = reset_color() +format_args!($($args)*).to_string().as_str();

        let log_message = format!("{}{}{} {}", time, prefix, perfix_2, msg);
        println!("{}", log_message.as_str());
    };
}

#[macro_export]
macro_rules! logger_error_assetloader {
    ($($args:tt)*) => {
        let color =  COLOR_RED;
        let prefix = set_color(color, "[ERROR] ");
        let perfix_2 = set_color(COLOR_YELLOW, *&ASSET_PREFIX.as_str());
        let time = set_color(COLOR_DARK_GREY, get_time().as_str())+ " ";
        let msg = reset_color() +format_args!($($args)*).to_string().as_str();

        let log_message = format!("{}{}{} {}", time, prefix, perfix_2, msg);
        println!("{}", log_message.as_str());
    };
}

//GameState Messages
#[macro_export]
macro_rules! logger_info_gamestate {
    ($($args:tt)*) => {
        let color = COLOR_GREEN;
        let prefix = set_color(color, "[INFO] ");
        let perfix_2 = set_color(COLOR_BLUE, "[GameState] ");
        let time = set_color(COLOR_DARK_GREY, get_time().as_str())+ " ";
        let msg = reset_color() +format_args!($($args)*).to_string().as_str();

        let log_message = format!("{}{}{} {}", time, prefix, perfix_2, msg);
        println!("{}", log_message.as_str());
    };
}

#[macro_export]
macro_rules! logger_warn_gamestate {
    ($($args:tt)*) => {
        let color = COLOR_YELLOW;
        let prefix = set_color(color, "[WARN] ");
        let perfix_2 = set_color(COLOR_BLUE, "[GameState] ");
        let time = set_color(COLOR_DARK_GREY, get_time().as_str())+ " ";
        let msg = reset_color() +format_args!($($args)*).to_string().as_str();

        let log_message = format!("{}{}{} {}", time, prefix, perfix_2, msg);
        println!("{}", log_message.as_str());
    };
}

#[macro_export]
macro_rules! logger_error_gamestate {
    ($($args:tt)*) => {
        let color =  COLOR_RED;
        let prefix = set_color(color, "[ERROR] ");
        let perfix_2 = set_color(COLOR_BLUE, "[GameState] ");
        let time = set_color(COLOR_DARK_GREY, get_time().as_str())+ " ";
        let msg = reset_color() +format_args!($($args)*).to_string().as_str();

        let log_message = format!("{}{}{}{}", time, prefix, perfix_2, msg);
        println!("{}", log_message.as_str());
    };
}