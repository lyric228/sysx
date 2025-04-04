pub mod io {
    pub mod cmd;
    pub mod env;
    pub mod log;
}
pub mod net {
    pub mod ipv4;
    pub mod ipv6;
}
pub mod math {
    pub mod math;
    pub use math::*;
    pub mod bin;
    pub mod hex;
}
pub mod utils {
    pub mod ascii;
    pub mod rand;
}
pub mod types {
    pub mod error;
    pub mod types;
    pub use types::*;
}
pub mod time {
    pub mod time;
    pub use time::*;
}

pub use types::error::*;

pub mod deps {
    pub use anyhow;
    pub use chrono;
    pub use colored;
    pub use lazy_static;
    pub use once_cell;
    pub use rand;
    pub use regex;
    pub use shell_words;
    pub use thiserror;
}
