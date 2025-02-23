use sysx::ipv4::*;


// is_valid_ipv4 tests

#[test]
// Тест валидации корректного IPv4 адреса.
// Проверяет успешную валидацию правильного адреса и порта.
fn test_is_valid_ipv4_correct() {
    assert!(is_valid_ipv4("192.168.0.1:8080"));
    assert!(is_valid_ipv4("127.0.0.1:80"));
    assert!(is_valid_ipv4("0.0.0.0:0"));
    assert!(is_valid_ipv4("255.255.255.255:65535"));
}

#[test]
// Тест валидации некорректных IPv4 адресов.
// Проверяет различные случаи неправильного формата.
fn test_is_valid_ipv4_incorrect() {
    assert!(!is_valid_ipv4("192.168.0.1")); // отсутствует порт
    assert!(!is_valid_ipv4("192.168.0:8080")); // неполный IP
    assert!(!is_valid_ipv4("192.168.0.1:65536")); // порт больше допустимого
    assert!(!is_valid_ipv4("192.168.0.256:8080")); // октет больше 255
    assert!(!is_valid_ipv4("192.168.0.1:")); // пустой порт
    assert!(!is_valid_ipv4(":8080")); // пустой IP
    assert!(!is_valid_ipv4("192.168.0.1:abc")); // некорректный порт
}

#[test]
// Тест валидации граничных значений.
// Проверяет работу с крайними допустимыми значениями.
fn test_is_valid_ipv4_edge_cases() {
    assert!(is_valid_ipv4("0.0.0.0:0")); // минимальные значения
    assert!(is_valid_ipv4("255.255.255.255:65535")); // максимальные значения
    assert!(!is_valid_ipv4("256.256.256.256:65536")); // превышение максимальных значений
}


// str_to_ipv4 tests

#[test]
// Тест преобразования корректного IPv4 адреса.
// Проверяет правильность создания SocketAddrV4.
fn test_str_to_ipv4_correct() {
    let addr = str_to_ipv4("127.0.0.1:8080").unwrap();
    assert_eq!(addr.ip(), &Ipv4Addr::new(127, 0, 0, 1));
    assert_eq!(addr.port(), 8080);
}

#[test]
// Тест преобразования некорректных адресов.
// Проверяет возврат None для различных неправильных форматов.
fn test_str_to_ipv4_incorrect() {
    assert!(str_to_ipv4("192.168.0.1").is_none()); // без порта
    assert!(str_to_ipv4("192.168.0:8080").is_none()); // неполный IP
    assert!(str_to_ipv4("192.168.0.1:65536").is_none()); // недопустимый порт
    assert!(str_to_ipv4("192.168.0.256:8080").is_none()); // недопустимый октет
}

#[test]
// Тест преобразования граничных значений.
// Проверяет работу с минимальными и максимальными значениями.
fn test_str_to_ipv4_edge_cases() {
    // Минимальные значения
    let min_addr = str_to_ipv4("0.0.0.0:0").unwrap();
    assert_eq!(min_addr.ip(), &Ipv4Addr::new(0, 0, 0, 0));
    assert_eq!(min_addr.port(), 0);

    // Максимальные значения
    let max_addr = str_to_ipv4("255.255.255.255:65535").unwrap();
    assert_eq!(max_addr.ip(), &Ipv4Addr::new(255, 255, 255, 255));
    assert_eq!(max_addr.port(), 65535);
}

#[test]
// Тест преобразования специальных адресов.
// Проверяет работу с локальными и специальными адресами.
fn test_str_to_ipv4_special_addresses() {
    // Localhost
    let localhost = str_to_ipv4("127.0.0.1:80").unwrap();
    assert_eq!(localhost.ip(), &Ipv4Addr::LOCALHOST);
    
    // Broadcast
    let broadcast = str_to_ipv4("255.255.255.255:8080").unwrap();
    assert_eq!(broadcast.ip(), &Ipv4Addr::BROADCAST);
    
    // Unspecified
    let unspecified = str_to_ipv4("0.0.0.0:8080").unwrap();
    assert_eq!(unspecified.ip(), &Ipv4Addr::UNSPECIFIED);
}

#[test]
// Тест преобразования с некорректными разделителями.
// Проверяет обработку неправильных форматов разделения.
fn test_str_to_ipv4_invalid_separators() {
    assert!(str_to_ipv4("192,168,0,1:8080").is_none()); // запятые вместо точек
    assert!(str_to_ipv4("192.168.0.1;8080").is_none()); // точка с запятой вместо двоеточия
    assert!(str_to_ipv4("192.168.0.1/8080").is_none()); // слеш вместо двоеточия
}
