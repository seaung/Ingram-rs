mod cli;
mod config;
mod core;
mod data;
mod fingerprint;
mod net;
mod poc;
mod pocs;
mod utils;

use anyhow::Result;
use log::LevelFilter;
use std::path::Path;
use std::sync::Arc;

fn setup_logging(debug: bool, out_dir: &str) -> Result<()> {
    let log_level = if debug {
        LevelFilter::Debug
    } else {
        LevelFilter::Info
    };

    let log_file = Path::new(out_dir).join("log.txt");
    
    env_logger::Builder::new()
        .filter_level(log_level)
        .write_style(env_logger::WriteStyle::Always)
        .format_timestamp_millis()
        .init();

    Ok(())
}

fn run() -> Result<()> {
    let args = cli::parse_args()?;

    utils::print_logo();

    let config = Arc::new(config::Config::new(
        args.in_file.clone(),
        args.out_dir.clone(),
        args.ports,
        args.th_num,
        args.timeout,
        args.disable_snapshot,
        args.debug,
    )?);

    if !Path::new(&config.in_file).exists() {
        utils::print_error(&format!(
            "The input file {} does not exist!",
            config.in_file
        ));
        std::process::exit(1);
    }

    setup_logging(config.debug, &config.out_dir)?;

    let mut core = core::Core::new(config)?;
    core.run()?;

    Ok(())
}

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
