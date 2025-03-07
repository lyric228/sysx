pub mod io {
    pub mod cmd;
    pub mod env;
    pub mod fs;
    pub mod log;
}
pub mod net {
    pub mod ipv4;
}
pub mod math {
    pub mod types {
        pub mod float {
            #[cfg(feature = "unstable")]
            pub mod f128;
            #[cfg(feature = "unstable")]
            pub mod f16;

            pub mod f32;
            pub mod f64;
        }

        #[cfg(feature = "unstable")]
        pub use float::f16;
        pub use float::f32;
        pub use float::f64;
        #[cfg(feature = "unstable")]
        pub use float::f128;
    }

    #[cfg(feature = "unstable")]
    pub use types::f16;
    pub use types::f32;
    pub use types::f64;
    #[cfg(feature = "unstable")]
    pub use types::f128;

    pub mod math;
    pub use math::*;
    pub mod bin;
    pub mod hex;
}
pub mod utils {
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
