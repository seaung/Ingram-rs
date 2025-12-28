use crate::config::Config;
use crate::poc::{Poc, PocResult};

pub struct BasePoc {
    name: String,
    product: String,
}

impl BasePoc {
    pub fn new(name: &str, product: &str) -> Self {
        BasePoc {
            name: name.to_string(),
            product: product.to_string(),
        }
    }
}

impl Poc for BasePoc {
    fn name(&self) -> &str {
        &self.name
    }

    fn product(&self) -> &str {
        &self.product
    }

    fn verify(&self, _ip: &str, _port: u16, _config: &Config) -> Option<PocResult> {
        None
    }

    fn exploit(&self, _result: &PocResult, _config: &Config) -> anyhow::Result<usize> {
        Ok(0)
    }
}
