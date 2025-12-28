use anyhow::{Context, Result};
use ipnetwork::IpNetwork;
use log::error;
use std::net::{IpAddr, SocketAddr, TcpStream};
use std::time::Duration;
use std::vec::Vec;

pub fn port_scan(ip: &str, port: u16, timeout: u64) -> bool {
    let address = format!("{}:{}", ip, port);
    let socket_addr: SocketAddr = match address.parse() {
        Ok(addr) => addr,
        Err(e) => {
            error!("Invalid address {}: {}", address, e);
            return false;
        }
    };

    let timeout_duration = Duration::from_secs(timeout);

    match TcpStream::connect_timeout(&socket_addr, timeout_duration) {
        Ok(_) => true,
        Err(_) => false,
    }
}

pub fn get_ip_seg_len(target: &str) -> usize {
    if let Ok(ip_network) = target.parse::<IpNetwork>() {
        if ip_network.is_ipv4() {
            let prefix = ip_network.prefix();
            let size = 2u32.pow(32 - prefix as u32);
            size as usize
        } else {
            1
        }
    } else if target.contains('-') {
        let parts: Vec<&str> = target.split('-').collect();
        if parts.len() == 2 {
            if let (Ok(start), Ok(end)) = (
                parts[0].parse::<IpAddr>(),
                parts[1].parse::<IpAddr>(),
            ) {
                if let (IpAddr::V4(start_v4), IpAddr::V4(end_v4)) = (start, end) {
                    let start_u32 = u32::from(start_v4);
                    let end_u32 = u32::from(end_v4);
                    if end_u32 >= start_u32 {
                        return (end_u32 - start_u32 + 1) as usize;
                    }
                }
            }
        }
        1
    } else if target.contains(':') {
        let parts: Vec<&str> = target.split(':').collect();
        if parts.len() == 2 {
            1
        } else {
            1
        }
    } else {
        1
    }
}

pub fn get_all_ip(target: &str) -> Vec<String> {
    let mut ips = Vec::new();

    if let Ok(ip_network) = target.parse::<IpNetwork>() {
        if ip_network.is_ipv4() {
            for ip in ip_network.iter() {
                ips.push(ip.to_string());
            }
        } else {
            ips.push(ip_network.ip().to_string());
        }
    } else if target.contains('-') {
        let parts: Vec<&str> = target.split('-').collect();
        if parts.len() == 2 {
            if let (Ok(start), Ok(end)) = (
                parts[0].parse::<IpAddr>(),
                parts[1].parse::<IpAddr>(),
            ) {
                if let (IpAddr::V4(start_v4), IpAddr::V4(end_v4)) = (start, end) {
                    let start_u32 = u32::from(start_v4);
                    let end_u32 = u32::from(end_v4);
                    if end_u32 >= start_u32 {
                        for i in start_u32..=end_u32 {
                            let ip = std::net::Ipv4Addr::from(i);
                            ips.push(ip.to_string());
                        }
                    }
                }
            }
        } else {
            ips.push(target.to_string());
        }
    } else if target.contains(':') {
        let parts: Vec<&str> = target.split(':').collect();
        if parts.len() == 2 {
            ips.push(parts[0].to_string());
        } else {
            ips.push(target.to_string());
        }
    } else {
        ips.push(target.to_string());
    }

    ips
}

pub fn parse_target(target: &str) -> Option<(String, Option<u16>)> {
    if target.contains(':') {
        let parts: Vec<&str> = target.split(':').collect();
        if parts.len() == 2 {
            let ip = parts[0].to_string();
            let port = parts[1].parse::<u16>().ok();
            Some((ip, port))
        } else {
            None
        }
    } else {
        Some((target.to_string(), None))
    }
}
