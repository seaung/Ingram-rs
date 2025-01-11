pub fn check_os() -> String {
    match std::env::consts::OS {
        "windows" => "windows".to_string(),
        "macos" => "mac".to_string(),
        "linux" => "linux".to_string(),
        _ => "other".to_string(),
    }
}