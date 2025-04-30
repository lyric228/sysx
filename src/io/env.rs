/// Returns a vector of command-line arguments, including the program name.
pub fn get_full_args() -> Vec<String> {
    std::env::args().collect()
}

/// Returns a vector of command-line arguments, excluding the program name.
pub fn get_args() -> Vec<String> {
    std::env::args().skip(1).collect()
}

/// Returns all command-line arguments as a single string, joined by spaces.
pub fn get_full_str_args() -> String {
    get_full_args().join(" ")
}

/// Returns command-line arguments (excluding program name) as a single string, joined by spaces.
pub fn get_str_args() -> String {
    get_args().join(" ")
}
