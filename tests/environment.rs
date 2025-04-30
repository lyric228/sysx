use sysx::io::env::*;

#[test]
fn test_arguments_handling() {
    let full_args = get_full_args();
    let args = get_args();

    assert!(!full_args.is_empty());
    assert_eq!(args.len(), full_args.len() - 1);
}
