use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rule {
    pub product: String,
    pub path: String,
    pub val: String,
}

#[derive(Debug, Clone)]
pub struct Config {
    pub users: Vec<String>,
    pub passwords: Vec<String>,
    pub user_agent: String,
    pub ports: Vec<u16>,
    pub rules: Vec<Rule>,
    pub product: HashSet<String>,
    pub log: String,
    pub not_vulnerable: String,
    pub vulnerable: String,
    pub snapshots: String,
    pub in_file: String,
    pub out_dir: String,
    pub th_num: usize,
    pub timeout: u64,
    pub disable_snapshot: bool,
    pub debug: bool,
}

impl Config {
    pub fn new(
        in_file: String,
        out_dir: String,
        ports: Option<Vec<u16>>,
        th_num: Option<usize>,
        timeout: Option<u64>,
        disable_snapshot: bool,
        debug: bool,
    ) -> Result<Self> {
        let user_agent = Self::get_user_agent();

        let default_ports = vec![
            80, 81, 82, 83, 84, 85, 88, 8000, 8001, 8080, 8081, 8085, 8086, 8088, 8090,
            8181, 2051, 9000, 37777, 49152, 55555,
        ];

        let users = vec!["admin".to_string()];
        let passwords = vec![
            "admin".to_string(),
            "admin12345".to_string(),
            "asdf1234".to_string(),
            "abc12345".to_string(),
            "12345admin".to_string(),
            "12345abc".to_string(),
        ];

        let rules = Self::load_rules()?;
        let product: HashSet<String> = rules.iter().map(|r| r.product.clone()).collect();

        Ok(Config {
            users,
            passwords,
            user_agent,
            ports: ports.unwrap_or(default_ports),
            rules,
            product,
            log: "log.txt".to_string(),
            not_vulnerable: "not_vulnerable.csv".to_string(),
            vulnerable: "results.csv".to_string(),
            snapshots: "snapshots".to_string(),
            in_file,
            out_dir,
            th_num: th_num.unwrap_or(300),
            timeout: timeout.unwrap_or(3),
            disable_snapshot,
            debug,
        })
    }

    fn get_user_agent() -> String {
        ua_generator::ua::spoof_ua().to_string()
    }

    fn load_rules() -> Result<Vec<Rule>> {
        let rules_path = Path::new("rules.csv");
        if !rules_path.exists() {
            return Ok(Vec::new());
        }

        let file = File::open(rules_path).context("Failed to open rules.csv")?;
        let reader = BufReader::new(file);
        let mut rdr = csv::Reader::from_reader(reader);
        let mut rules = Vec::new();

        for result in rdr.deserialize() {
            let record: (String, String, String) = result.context("Failed to parse CSV record")?;
            rules.push(Rule {
                product: record.0,
                path: record.1,
                val: record.2,
            });
        }

        Ok(rules)
    }
}
