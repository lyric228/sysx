use sysx::io::env::*;


#[test]
fn test_environment_manipulation() {
    // set_env
    set_env("TEST_KEY", "test_value").unwrap();
    assert_eq!(get_env("TEST_KEY").unwrap(), "test_value");
    
    // get_envs
    let envs = get_envs();
    assert!(envs.contains_key("TEST_KEY"));
}

#[test]
fn test_arguments_handling() {
    let full_args = get_full_args();
    let args = get_args();
    
    assert!(!full_args.is_empty());
    assert_eq!(args.len(), full_args.len() - 1);
}
