pub mod io;
pub mod fs;
pub mod proc;
pub mod utils;

pub mod linux {
    pub mod system;
}

pub mod windows {

}

pub mod types {
    mod types;
    pub use types::*;
}
