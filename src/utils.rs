use colored::Colorize;

pub fn print_logo() {
    let logo = [
        (
            r#"
  _____  _                 _    _            
 |  __ \(_)               | |  (_)           
 | |  | |_ _ __ ___   ___ | | ___  ___ _ __  
 | |  | | | '_ ` _ \ / _ \| |/ / |/ _ \ '_ \ 
 | |__| | | | | | | | (_) |   <| |  __/ | | |
 |_____/|_|_| |_| |_|\___/|_|\_\_|\___|_| |_|
"#,
            "Webcam Vulnerability Scanner",
        ),
    ];

    for (icon, font) in logo.iter() {
        println!("{}  {}", icon.bright_yellow(), font.bright_magenta());
    }
}

pub fn print_error(msg: &str) {
    eprintln!("{}", msg.red());
}

pub fn print_warning(msg: &str) {
    println!("{}", msg.yellow());
}

pub fn print_success(msg: &str) {
    println!("{}", msg.green());
}

pub fn print_info(msg: &str) {
    println!("{}", msg.cyan());
}
