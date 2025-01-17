use std::collections::HashMap;
use std::time::Duration;
use reqwest::Response;

#[derive(Debug)]
struct Rule {
    path: String,
    val: String,
    product: String,
}

#[derive(Debug)]
struct Configs {
    user_agent: String,
    timeout: u64,
    rules: Vec<Rule>,
}

pub fn parse(resp: &Response, rule_val: &str) -> bool {
    let checks: Vec<&str> = rule_val.split("&&").collect();
    checks.iter().all(|&rule| check_one(resp, rule))
}

fn check_one(resp: &Response, rule_val: &str) -> bool {
    false
}

fn fingerprint(ip: &str, port: u16, config: &Configs) -> Option<String> {
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(config.timeout))
        .build().unwrap();

    let mut req_dict: HashMap<String, Response> = HashMap::new();

    for rule in &config.rules {
        let url = format!("http://{}:{}/{}", ip, port, rule.path);
        if rule.contains_key(&url) {
            match client.get(&url).header("User-Agent", &config.user_agent).send() {
                Ok(req) => {
                    if req.status().is_success() {
                        req_dict.insert(rule.path.clone(), req);
                    }
                },
                Err(e) => {
                    continue;
                }
            }
        }

        if let Some(req) = req_dict.get(&rule.path) {
            if parse(rule, &req_dict) {
                return Some(rule.product.clone());
            }
        }
    }
    None
}