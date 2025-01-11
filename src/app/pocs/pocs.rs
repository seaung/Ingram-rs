use std::path::PathBuf;

#[derive(Debug)]
pub enum Level {
    High,
    Medium,
    Low,
}

#[derive(Debug)]
pub struct PocConfig {
    pub output_directory: PathBuf,
    pub snapshots: PathBuf,
    pub user_agent: String,
    pub timeout: u64,
}

#[derive(Debug)]
pub struct PocTemplate {
    config: PocConfig,
    name: String,
    product: String,
    product_version: String,
    ref_url: String,
    level: Level,
    desc: String,
}

#[derive(Debug)]
pub struct PocResult {
    ip: String,
    port: String,
    protocol: String,
    password: String,
    name: String,
}

pub trait PocBase {
    fn verify(&self, ip: &str, port: u16) -> Option<PocResult>;
    fn exploit(&self, results: (String, u16, String, String, String, String)) -> i32;
    fn snapshot(&self, url: &str, img_file_name: &str, auth: Option<(&str, &str)>) -> i32;
}

impl PocTemplate {
    fn new(config: PocConfig, name: &str,
        product: &str, version: &str,
        level: &str, desc: &str, reference: &str) -> Self {
        Self{
            config,
            name: name.to_string(),
            product: product.to_string(),
            product_version: version.to_string(),
            ref_url: "".to_string(),
            level: Level::Medium,
            desc: "".to_string(),
        }
    }
}