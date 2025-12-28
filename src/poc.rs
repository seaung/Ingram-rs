use crate::config::Config;
use anyhow::Result;
use reqwest::blocking::Client;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::sync::Arc;

pub struct PocResult {
    pub ip: String,
    pub port: u16,
    pub product: String,
    pub user: String,
    pub password: String,
    pub vul_name: String,
}

pub trait Poc: Send + Sync + 'static {
    fn name(&self) -> &str;
    fn product(&self) -> &str;
    fn verify(&self, ip: &str, port: u16, config: &Config) -> Option<PocResult>;
    fn exploit(&self, result: &PocResult, config: &Config) -> Result<usize>;
}

pub fn to_exploit_func(poc: Arc<dyn Poc>) -> Arc<dyn Fn(&PocResult, &Config) -> Result<usize> + Send + Sync> {
    Arc::new(move |result, config| poc.exploit(result, config))
}

pub fn download_snapshot(
    url: &str,
    file_path: &str,
    config: &Config,
    username: Option<&str>,
    password: Option<&str>,
) -> Result<usize> {
    let client = Client::builder()
        .timeout(std::time::Duration::from_secs(config.timeout))
        .build()?;

    let mut request = client.get(url);
    request = request.header("User-Agent", &config.user_agent);
    request = request.header("Connection", "close");

    if let (Some(user), Some(pass)) = (username, password) {
        request = request.basic_auth(user, Some(pass));
    }

    let response = request.send()?;

    if response.status().as_u16() != 200 {
        return Ok(0);
    }

    let content = response.bytes()?;

    let path = Path::new(file_path);
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }

    let mut file = File::create(file_path)?;
    file.write_all(&content)?;

    Ok(1)
}
