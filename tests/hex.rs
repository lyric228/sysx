use sysx::math::hex::*;

#[test]
fn test_hex_clean() {
    assert_eq!(clean("1a2b!@#"), "1a2b");
    assert_eq!(clean("DEADBEEF"), "DEADBEEF");
    assert_eq!(clean(""), "");
}

#[test]
fn test_hex_case_conversion() {
    assert_eq!(to_uppercase("deadBEEF"), "DEADBEEF");
    assert_eq!(to_lowercase("DEADbeef"), "deadbeef");
}

#[test]
fn test_hex_decode() {
    // Valid cases
    assert_eq!(decode("48656C6C6F").unwrap(), "Hello");
    assert_eq!(decode("48 65 6C 6C 6F").unwrap(), "Hello");
    assert_eq!(decode("48z65$6C\n6C_6F").unwrap(), "Hello");
    
    // Error cases
    assert!(decode("486").is_err());
    assert!(decode("48GG").is_err());
}

#[test]
fn test_hex_encode() {
    assert_eq!(encode("A"), "41");
    assert_eq!(encode("AB"), "41 42");
}

#[test]
fn test_hex_format() {
    assert_eq!(format("48656C6C6F").unwrap(), "48 65 6C 6C 6F");
    assert!(format("123").is_err());
}

#[test]
fn test_hex_validation() {
    assert!(is_valid("CAFE B0BA"));
    assert!(!is_valid("CAFE Z0BA"));
    assert!(is_valid_strict("DEADBEEF"));
    assert!(!is_valid_strict("DEADBEE"));
}

#[test]
fn test_hex_round_trip() {
    let original = "Hello World!";
    let encoded = encode(original);
    let decoded = decode(&encoded).unwrap();
    assert_eq!(decoded, original);
    
    let formatted = format(&clean(&encoded)).unwrap();
    let decoded_formatted = decode(&formatted).unwrap();
    assert_eq!(decoded_formatted, original);
}
