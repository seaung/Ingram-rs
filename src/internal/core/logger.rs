use chrono::Local;
use console::Style;

fn logger_message(level: &str, color: Style, message: &str) {
    let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    println!("{} [{}] {}", timestamp, color.apply_to(level), message);
}

pub fn logger_warning(message: &str) {
    let style = Style::new().yellow().bold();
    logger_message("WARN", style, message);
}

pub fn logger_error(message: &str) {
    let style = Style::new().red().bold();
    logger_message("ERR", style, message);
}

pub fn logger_info(message: &str) {
    let style = Style::new().blue().bold();
    logger_message("INFO", style, message);
}

pub fn logger_debug(message: &str) {
    let style = Style::new().cyan().bold();
    logger_message("DEBUG", style, message);
}