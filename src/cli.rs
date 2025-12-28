use anyhow::Result;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "ingram-rs")]
#[command(about = "A webcam vulnerability scanning framework written in Rust", long_about = None)]
pub struct Args {
    #[arg(short, long, help = "The targets file to scan")]
    pub in_file: String,

    #[arg(short, long, help = "The output directory for results")]
    pub out_dir: String,

    #[arg(short, long, value_delimiter = ',', help = "The port(s) to detect")]
    pub ports: Option<Vec<u16>>,

    #[arg(short = 't', long, default_value = "300", help = "The number of threads")]
    pub th_num: Option<usize>,

    #[arg(short = 'T', long, default_value = "3", help = "Request timeout in seconds")]
    pub timeout: Option<u64>,

    #[arg(short = 'D', long, action = clap::ArgAction::SetTrue, help = "Disable snapshot")]
    pub disable_snapshot: bool,

    #[arg(long, action = clap::ArgAction::SetTrue, help = "Enable debug mode")]
    pub debug: bool,
}

pub fn parse_args() -> Result<Args> {
    Ok(Args::parse())
}
