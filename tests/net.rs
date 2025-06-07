use std::net::{Ipv4Addr, Ipv6Addr};

use sysx::net::{ipv4::*, ipv6::*};

#[test]
fn test_ipv4_validation() {
    assert!(is_valid_ipv4("192.168.0.1:8080"));
    assert!(is_valid_ipv4("127.0.0.1:80"));
    assert!(is_valid_ipv4("0.0.0.0:443"));

    assert!(!is_valid_ipv4("192.168.0.256:8080"));
    assert!(!is_valid_ipv4("192.168.0.1"));
    assert!(!is_valid_ipv4("192.168.0.1:65536"));
    assert!(!is_valid_ipv4("192.168.0.1:-1"));
    assert!(!is_valid_ipv4("192.168.0.1:abc"));
    assert!(!is_valid_ipv4("abc:80"));
}

#[test]
fn test_ipv4_parsing() {
    let addr = str_to_ipv4("127.0.0.1:80").unwrap();
    assert_eq!(addr.ip(), &Ipv4Addr::LOCALHOST);
    assert_eq!(addr.port(), 80);

    let addr = str_to_ipv4("192.168.1.10:8080").unwrap();
    assert_eq!(addr.ip(), &Ipv4Addr::new(192, 168, 1, 10));
    assert_eq!(addr.port(), 8080);

    assert_eq!(str_to_ipv4("192.168.0.1"), None);
    assert_eq!(str_to_ipv4("300.168.0.1:8080"), None);
}

#[test]
fn test_ipv6_validation() {
    assert!(is_valid_ipv6("[::1]:8080"));
    assert!(is_valid_ipv6("[2001:db8::1]:80"));
    assert!(is_valid_ipv6("[fe80::1]:443"));

    assert!(!is_valid_ipv6("::1:8080"));
    assert!(!is_valid_ipv6("[::1]"));
    assert!(!is_valid_ipv6("[::1]:65536"));
    assert!(!is_valid_ipv6("[::gggg:80"));
}

#[test]
fn test_ipv6_parsing() {
    let addr = str_to_ipv6("[::1]:80").unwrap();
    assert_eq!(addr.ip(), &Ipv6Addr::LOCALHOST);
    assert_eq!(addr.port(), 80);

    let addr = str_to_ipv6("[2001:db8::1]:8080").unwrap();
    assert_eq!(addr.ip(), &Ipv6Addr::new(0x2001, 0xdb8, 0, 0, 0, 0, 0, 1));
    assert_eq!(addr.port(), 8080);

    assert_eq!(str_to_ipv6("[::1]"), None);
    assert_eq!(str_to_ipv6("::1:8080"), None);
}
