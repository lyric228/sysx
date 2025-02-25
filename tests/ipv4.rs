use sysx::net::ipv4::*;


#[test]
fn test_ipv4_validation() {
    assert!(is_valid_ipv4("192.168.0.1:8080"));
    assert!(!is_valid_ipv4("192.168.0.256:8080"));
    
    let addr = str_to_ipv4("127.0.0.1:80").unwrap();
    assert_eq!(addr.ip(), &Ipv4Addr::LOCALHOST);
}
