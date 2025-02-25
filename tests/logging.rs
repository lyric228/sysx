use sysx::io::log::*;


#[test]
fn test_log_levels() {
    assert_eq!(LogLevel::Info.style(), Color::Blue);
    assert_eq!(log_level!(WARNING), LogLevel::Warning);
}

#[test]
fn test_log_formatting() {
    let styled = style!("test", LogLevel::Warning);
    assert_eq!(styled.fgcolor, Some(Color::Yellow));
}
