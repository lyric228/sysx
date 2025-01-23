pub mod sys;
pub mod fs;
pub mod proc;
pub mod test;
pub mod utils;

pub mod linux {
    pub mod info;
}

pub mod windows {

}

pub mod types {
    pub mod types;
    pub mod better_hash_map;

    pub use self::better_hash_map::BHashMap;
    pub use self::types::*;
}
