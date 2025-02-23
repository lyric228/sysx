pub use std::net::{
    Ipv4Addr,
    SocketAddrV4,
};


/// Проверяет, является ли строка допустимым IPv4 адресом с указанием порта.
///
/// Функция принимает строку в формате "IP:PORT", где IP имеет вид "x.x.x.x", 
/// а PORT – число от 0 до 65535. Сначала проверяются разделители и корректность формата, 
/// затем валидируются отдельные октеты и порт.
/// 
/// # Возвращаемое значение
/// Возвращает true, если строка соответствует формату допустимого IPv4 адреса с портом, иначе false.
/// 
/// # Пример
/// ```
/// // Валидный адрес:
/// let valid = is_valid_ipv4("192.168.0.1:8080");
/// // valid будет true
///
/// // Невалидный адрес:
/// let invalid = is_valid_ipv4("192.168.0.1");
/// // invalid будет false
/// ```
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
        if octet.is_empty() {
            return false;
        }
        let _num: u8 = match octet.parse() {
            Ok(n) => n,
            Err(_) => return false,
        };
    }
    
    true
}

/// Преобразует строку в SocketAddrV4, если строка является корректным IPv4 адресом с портом.
///
/// Функция сначала проверяет строку с помощью is_valid_ipv4. Затем парсит IP-адрес и порт, 
/// и в случае успешного преобразования возвращает Some(SocketAddrV4). Если преобразование не удалось, 
/// возвращается None.
///
/// # Возвращаемое значение
/// Some(SocketAddrV4) при успешном преобразовании или None, если строка некорректна.
/// 
/// # Пример
/// ```
/// // Преобразование корректного адреса:
/// let addr = str_to_ipv4("127.0.0.1:3000");
/// // addr будет Some(SocketAddrV4) с IP 127.0.0.1 и портом 3000
///
/// // Преобразование некорректного адреса:
/// let addr_invalid = str_to_ipv4("127.0.0.1");
/// // addr_invalid будет None
/// ```
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

    Some(SocketAddrV4::new(
        Ipv4Addr::new(octets[0], octets[1], octets[2], octets[3]),
        port,
    ))
}
