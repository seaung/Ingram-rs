use regex::Regex;
use reqwest::Response;

#[derive(Debug)]
struct Rule {
    path: String,
    val: String,
    product: String,
}

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