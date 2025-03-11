use std::net::{Ipv6Addr, SocketAddrV6};

/// Проверяет, является ли строка допустимым IPv6 адресом с указанием порта.
///
/// # Пример
/// ```
/// use sysx::net::ipv6::is_valid_ipv6;
///
/// assert!(is_valid_ipv6("[::1]:8080"));
/// assert!(is_valid_ipv6("[2001:db8::1]:80"));
/// assert!(is_valid_ipv6("[fe80::1]:443"));
///
/// assert!(!is_valid_ipv6("::1:8080")); // отсутствуют квадратные скобки
/// assert!(!is_valid_ipv6("[::1]")); // отсутствует порт
/// assert!(!is_valid_ipv6("[::1]:65536")); // порт вне диапазона
/// assert!(!is_valid_ipv6("[::gggg]:80")); // недопустимые символы
/// ```
pub fn is_valid_ipv6(s: &str) -> bool {
    s.parse::<SocketAddrV6>().is_ok()
}

/// Преобразует строку в SocketAddrV6, если строка является корректным IPv6 адресом с портом.
///
/// # Пример
/// ```
/// use sysx::net::ipv6::str_to_ipv6;
/// use std::net::{Ipv6Addr, SocketAddrV6};
///
/// let addr = str_to_ipv6("[::1]:80").unwrap();
/// assert_eq!(addr.ip(), &Ipv6Addr::LOCALHOST);
/// assert_eq!(addr.port(), 80);
///
/// assert!(str_to_ipv6("[::1]").is_none());
/// assert!(str_to_ipv6("::1:8080").is_none());
/// ```
pub fn str_to_ipv6(s: &str) -> Option<SocketAddrV6> {
    s.parse::<SocketAddrV6>().ok()
}

/// Создаёт SocketAddrV6 из IP-адреса, порта, flow info и scope ID.
///
/// # Пример
/// ```
/// use sysx::net::ipv6::create_ipv6_socket;
/// use std::net::{Ipv6Addr, SocketAddrV6};
///
/// let addr = create_ipv6_socket("::1", 8080, 0, 0).unwrap();
/// assert_eq!(addr, SocketAddrV6::new(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1), 8080, 0, 0));
///
/// assert!(create_ipv6_socket("::gggg", 8080, 0, 0).is_none());
/// ```
pub fn create_ipv6_socket(
    ip: &str,
    port: u16,
    flowinfo: u32,
    scope_id: u32,
) -> Option<SocketAddrV6> {
    let ip_addr = ip.parse::<Ipv6Addr>().ok()?;
    Some(SocketAddrV6::new(ip_addr, port, flowinfo, scope_id))
}
