use std::net::{IpAddr, Ipv4Addr};
use std::str::FromStr;
use trust_dns_resolver::Resolver;

pub fn check_os() -> String {
    match std::env::consts::OS {
        "windows" => "windows".to_string(),
        "macos" => "mac".to_string(),
        "linux" => "linux".to_string(),
        _ => "other".to_string(),
    }
}

pub fn check_host(addr: &str) -> Option<IpAddr> {
    if let Ok(addr) = Ipv4Addr::from_str(addr) {
        return Some(IpAddr::V4(addr));
    }

    let resolver = Resolver::from_system_conf().ok()?;
    match resolver.lookup_ip(addr) {
        Ok(res) => res.iter().next(),
        Err(_) => None,
    }
}

pub fn check_port(port: &str) -> bool {
    match port.parse::<u16>() {
        Ok(p) => p >= 1 && p <= 65535,
        Err(_) => false,
    }
}

pub fn check_ip(ip: &str) -> bool {
    let parts: Vec<&str> = ip.split('.').collect();

    if parts.len() != 4 {
        return false;
    }

    for item in parts {
        match item.parse::<u8>() {
            Ok(num) => {
                if num > 255 {
                    return false;
                }
            },
            Err(_) => return false,
        }
    }
    true
}

pub fn dahua_proto(proto: &[u8]) -> bool {
    let headers: [&[u8]; 8] = [
        &[0xa0, 0x00],  // 3DES Login
        &[0xa0, 0x01],  // DVRIP Send Request Realm
        &[0xa0, 0x05],  // DVRIP login Send Login Details
        &[0xb0, 0x00],  // DVRIP Receive
        &[0xb0, 0x01],  // DVRIP Receive
        &[0xa3, 0x01],  // DVRIP Discover Request
        &[0xb3, 0x00],  // DVRIP Discover Response
        &[0xf6, 0x00],  // DVRIP JSON
    ];

    proto.get(0..2).is_some_and(|p| headers.iter().any(|&h| h == p))
}