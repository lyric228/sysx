use terminal_size::{terminal_size, Height, Width};

pub fn terminal_dimensions() -> Option<(u16, u16)> {
    terminal_size().map(|(Width(w), Height(h))| (w, h))
}

pub fn terminal_width() -> Option<u16> {
    terminal_size().map(|(Width(w), _)| w)
}

pub fn terminal_height() -> Option<u16> {
    terminal_size().map(|(_, Height(h))| h)
}
