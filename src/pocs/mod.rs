pub mod base;
pub mod dahua;
pub mod hikvision;

use crate::config::Config;
use crate::poc::{Poc, PocResult};
use std::collections::HashMap;
use std::sync::Arc;

pub fn get_poc_dict(config: &Config) -> HashMap<String, Vec<Arc<dyn Poc>>> {
    let mut poc_dict: HashMap<String, Vec<Arc<dyn Poc>>> = HashMap::new();

    let dahua_pocs: Vec<Arc<dyn Poc>> = vec![
        Arc::new(dahua::DahuaWeakPassword::new(config)),
    ];

    let hikvision_pocs: Vec<Arc<dyn Poc>> = vec![
        Arc::new(hikvision::HikvisionWeakPassword::new(config)),
    ];

    poc_dict.insert("dahua".to_string(), dahua_pocs);
    poc_dict.insert("hikvision".to_string(), hikvision_pocs);

    poc_dict
}
