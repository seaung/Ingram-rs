use crate::config::Config;
use log::error;
use md5::{compute, Digest};
use regex::Regex;
use reqwest::blocking::Client;
use scraper::{Html, Selector};
use std::collections::HashMap;
use std::time::Duration;

pub fn fingerprint(ip: &str, port: u16, config: &Config) -> Option<String> {
    let client = Client::builder()
        .timeout(Duration::from_secs(config.timeout))
        .build()
        .ok()?;

    let mut req_dict: HashMap<String, String> = HashMap::new();

    for rule in &config.rules {
        let url = format!("http://{}:{}{}", ip, port, rule.path);

        let body = if let Some(cached_body) = req_dict.get(&rule.path) {
            cached_body
        } else {
            match client
                .get(&url)
                .header("User-Agent", &config.user_agent)
                .header("Connection", "close")
                .send()
            {
                Ok(response) => {
                    if response.status().as_u16() == 200 {
                        match response.text() {
                            Ok(text) => {
                                req_dict.insert(rule.path.clone(), text);
                                req_dict.get(&rule.path)?
                            }
                            Err(_) => continue,
                        }
                    } else {
                        continue;
                    }
                }
                Err(e) => {
                    error!("Request error for {}: {}", url, e);
                    continue;
                }
            }
        };

        if parse_rule(body, &rule.val) {
            return Some(rule.product.clone());
        }
    }

    None
}

fn parse_rule(body: &str, rule_val: &str) -> bool {
    let rules: Vec<&str> = rule_val.split("&&").collect();

    for rule in rules {
        if !check_one_rule(body, rule) {
            return false;
        }
    }

    true
}

fn check_one_rule(body: &str, rule: &str) -> bool {
    let re = Regex::new(r"(.*)=`(.*)`").unwrap();
    if let Some(caps) = re.captures(rule) {
        let left = caps.get(1).map(|m| m.as_str()).unwrap_or("");
        let right = caps.get(2).map(|m| m.as_str()).unwrap_or("");

        match left {
            "md5" => {
                let digest: Digest = compute(body.as_bytes());
                let hash = format!("{:x}", digest);
                hash == right
            }
            "title" => {
                let document = Html::parse_document(body);
                let selector = Selector::parse("title").unwrap();
                if let Some(title_element) = document.select(&selector).next() {
                    let title = title_element.text().collect::<String>().to_lowercase();
                    title.to_lowercase().contains(&right.to_lowercase())
                } else {
                    false
                }
            }
            "body" => {
                let document = Html::parse_document(body);
                let selector = Selector::parse("body").unwrap();
                if let Some(body_element) = document.select(&selector).next() {
                    let body_text = body_element.text().collect::<String>().to_lowercase();
                    body_text.to_lowercase().contains(&right.to_lowercase())
                } else {
                    false
                }
            }
            "headers" => {
                false
            }
            "status_code" => {
                false
            }
            _ => false,
        }
    } else {
        false
    }
}
