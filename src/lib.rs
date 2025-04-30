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
    mod types;
    pub use types::*;
}
pub mod time {
    mod time;
    pub use time::*;
}

pub use types::error::*;
