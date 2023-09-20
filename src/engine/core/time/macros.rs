#[macro_export]
macro_rules! get_current_time {
    () => {
        let current_time = Local::now().format("[%Y-%m-%d][%H:%M:%S]").to_string();
        current_time
    };
}