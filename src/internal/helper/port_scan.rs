use std::error::Error;
use std::net::TcpStream;
use std::time::Duration;

pub fn port_scan(ip: &str, port: &str, timeout: u64) -> Result<bool, Box<dyn Error>> {
    let addr = format!("{}:{}", ip, port);
    let timeout = Duration::from_secs(timeout as u64);

    match TcpStream::connect_timeout(&addr.parse()?, timeout) {
        Ok(_stream) => Ok(true),
        Err(ref e) if e.kind() == std::io::ErrorKind::TimedOut => Ok(false),
        Err(e) => Ok(e.kind() == std::io::ErrorKind::ConnectionRefused),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    // use std::sync::mpsc;
    use std::net::{TcpListener};
    use std::thread;
    use rand::random;

    #[test]
    fn test_port_scan_open() {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let addr = listener.local_addr().unwrap();

        let handle = thread::spawn(move || {
            for stream in listener.incoming() {
                let _stream = stream.unwrap();
                break;
            }
        });

        assert!(port_scan("127.0.0.1", &addr.port().to_string(), 10).unwrap());

        handle.join().unwrap();
    }

    #[test]
    fn test_port_scan_close() {
        let port = 49152 + (random::<u16>() % 1024);
        assert!(port_scan("127.0.0.1", &port.to_string(), 10).unwrap());
    }
}