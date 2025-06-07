use terminal_size::{Height, Width, terminal_size};

/// Get terminal width and height (x, y)
pub fn txy() -> Option<(u16, u16)> {
    terminal_size().map(|(Width(w), Height(h))| (w, h))
}

/// Get terminal width (x)
pub fn tx() -> Option<u16> {
    terminal_size().map(|(Width(w), _)| w)
}

/// Get terminal height (y)
pub fn ty() -> Option<u16> {
    terminal_size().map(|(_, Height(h))| h)
}
