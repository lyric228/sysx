use std::net::{Ipv4Addr, SocketAddrV4};

/// Проверяет, является ли строка допустимым IPv4 адресом с указанием порта.
///
/// # Пример
/// ```
/// use sysx::net::ipv4::is_valid_ipv4;
///
/// assert!(is_valid_ipv4("127.0.0.1:80"));
/// assert!(is_valid_ipv4("192.168.1.1:8080"));
///
/// assert!(!is_valid_ipv4("192.168.0.1")); // отсутствует порт
/// assert!(!is_valid_ipv4("192.168.0.256:8080")); // недопустимый октет
/// assert!(!is_valid_ipv4("192.168.0.1:65536")); // порт вне диапазона
/// ```
pub fn is_valid_ipv4(s: &str) -> bool {
    s.parse::<SocketAddrV4>().is_ok()
}

/// Преобразует строку в SocketAddrV4, если строка является корректным IPv4 адресом с портом.
///
/// # Пример
/// ```
/// use sysx::net::ipv4::str_to_ipv4;
/// use std::net::{Ipv4Addr, SocketAddrV4};
///
/// let addr = str_to_ipv4("127.0.0.1:80").unwrap();
/// assert_eq!(addr.ip(), &Ipv4Addr::LOCALHOST);
/// assert_eq!(addr.port(), 80);
///
/// assert!(str_to_ipv4("192.168.0.1").is_none());
/// ```
pub fn str_to_ipv4(s: &str) -> Option<SocketAddrV4> {
    s.parse::<SocketAddrV4>().ok()
}

/// Создаёт SocketAddrV4 из IP-адреса и порта.
///
/// # Пример
/// ```
/// use sysx::net::ipv4::create_ipv4_socket;
/// use std::net::{Ipv4Addr, SocketAddrV4};
///
/// let addr = create_ipv4_socket("192.168.1.1", 8080).unwrap();
/// assert_eq!(addr, SocketAddrV4::new(Ipv4Addr::new(192, 168, 1, 1), 8080));
///
/// assert!(create_ipv4_socket("300.168.1.1", 8080).is_none());
/// ```
pub fn create_ipv4_socket(ip: &str, port: u16) -> Option<SocketAddrV4> {
    let ip_addr = ip.parse::<Ipv4Addr>().ok()?;
    Some(SocketAddrV4::new(ip_addr, port))
}
