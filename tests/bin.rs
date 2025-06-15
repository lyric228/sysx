use sysx::math::bin::*;

#[test]
fn test_binary_clean() {
    assert_eq!(clean("01a2b3c"), "01");
    assert_eq!(clean("1100!@#"), "1100");
    assert_eq!(clean(""), "");
}

#[test]
fn test_binary_decode() {
    // Valid cases
    assert_eq!(decode("01001000").unwrap(), "H");
    assert_eq!(decode("0100000101000010").unwrap(), "AB");
    assert_eq!(decode("0100 1000 !@#").unwrap(), "H");
    
    // Error cases
    assert!(decode("").is_err());
    assert!(decode("010010").is_err());
    assert!(decode("01002A01").is_err());
}

#[test]
fn test_binary_encode() {
    assert_eq!(encode("H"), "01001000");
    assert_eq!(encode("AB"), "01000001 01000010");
}

#[test]
fn test_binary_format() {
    assert_eq!(format("0100000101000010").unwrap(), "01000001 01000010");
    assert!(format("010010").is_err());
}

#[test]
fn test_binary_validation() {
    assert!(is_valid("0100 1101"));
    assert!(!is_valid("0100 2101"));
    assert!(is_valid_strict("00000000"));
    assert!(!is_valid_strict("0000000"));
}

#[test]
fn test_binary_round_trip() {
    let original = "Test";
    let encoded = encode(original);
    let decoded = decode(&encoded).unwrap();
    assert_eq!(decoded, original);
    
    let formatted = format(&clean(&encoded)).unwrap();
    let decoded_formatted = decode(&formatted).unwrap();
    assert_eq!(decoded_formatted, original);
}
