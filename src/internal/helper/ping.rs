use std::time::Duration;
use ping;
use rand::random;

fn ping_helper(ip: &str, nums: u32, timeout: u64) -> bool {
    let addr = ip.parse().unwrap();
    let timeout = Duration::from_secs(timeout);
    ping::dgramsock::ping(addr, Some(timeout), Some(nums), Some(3), Some(5), Some(&random())).is_ok()
}

pub fn alive_check(ip: &str, timeout: u64) -> bool {
    ping_helper(ip, 166, timeout)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ping_helper() {
        let is_ok = alive_check("127.0.0.1", 1);
        println!("is_ok: {}", is_ok);

        assert_eq!(is_ok, true);
    }

    #[test]
    fn test_no_alive() {
        let is_ok = alive_check("192.168.100.1", 1);

        println!("is_ok: {}", is_ok);
        assert_eq!(is_ok, false);
        println!("------");
        assert_eq!(alive_check("192.168.1.1", 1), true);
    }
}