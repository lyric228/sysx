use sysx::utils::rand::*;

#[test]
fn test_random_generators() {
    let num = random(1, 10).unwrap();
    assert!((1..=10).contains(&num));

    let s = random_string(10, None).unwrap();
    assert_eq!(s.len(), 10);
}
