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

#[macro_export]
macro_rules! logger_info {
    ($($args:tt)*) => {
        let color = metadata::COLOR_GREEN;
        let prefix = set_color(color, "[WARN] ");
        let time = set_color(metadata::COLOR_DARK_GREY, get_time().as_str())+ " ";
        let msg = reset_color() +format_args!($($args)*).to_string().as_str();

        let log_message = format!("{}{}{}", time, prefix, msg);
        println!("{}", log_message.as_str());
    };
}

#[macro_export]
macro_rules! logger_warn {
    ($($args:tt)*) => {
        let color = metadata::COLOR_YELLOW;
        let prefix = set_color(color, "[WARN] ");
        let time = set_color(metadata::COLOR_DARK_GREY, get_time().as_str())+ " ";
        let msg = reset_color() +format_args!($($args)*).to_string().as_str();

        let log_message = format!("{}{}{}", time, prefix, msg);
        println!("{}", log_message.as_str());
    };
}

#[macro_export]
macro_rules! logger_error {
    ($($args:tt)*) => {
        let color =  metadata::COLOR_RED;
        let prefix = "Error";
        let prefix = set_color(color, "[ERROR] ");
        let time = set_color(metadata::COLOR_DARK_GREY, get_time().as_str())+ " ";
        let msg = reset_color() +format_args!($($args)*).to_string().as_str();

        let log_message = format!("{}{}{}", time, prefix, msg);
        println!("{}", log_message.as_str());
    };
}