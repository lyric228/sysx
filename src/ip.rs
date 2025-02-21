use std::net::{Ipv4Addr, SocketAddrV4};


pub fn is_valid_ipv4(s: &str) -> bool {
    let parts: Vec<&str> = s.split(':').collect();

    if parts.len() != 2 {
        return false;
    }
    
    let ip_str = parts[0];
    let port_str = parts[1];
    
    let _port: u16 = match port_str.parse() {
        Ok(n) => n,
        Err(_) => return false,
    };

    let ip_parts: Vec<&str> = ip_str.split('.').collect();
    if ip_parts.len() != 4 {
        return false;
    }
    
    for octet in ip_parts {
        let _num: u8 = match octet.parse() {
            Ok(n) => n,
            Err(_) => return false,
        };

        if octet.is_empty() {
            return false;
        }
    }
    
    true
}


pub fn str_to_ipv4(s: &str) -> Option<SocketAddrV4> {
    if !is_valid_ipv4(s) {
        return None;
    }

    let parts: Vec<&str> = s.split(':').collect();
    let ip_str = parts[0];
    let port: u16 = parts[1].parse().ok()?;
    let octets: Vec<u8> = ip_str.split('.')
        .map(|octet| octet.parse::<u8>())
        .collect::<Result<Vec<u8>, _>>()
        .ok()?;

    if octets.len() != 4 {
        return None;
    }

    Some(SocketAddrV4::new(Ipv4Addr::new(octets[0], octets[1], octets[2], octets[3]), port))
}
