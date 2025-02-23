use std::collections::HashMap;
use lazy_static::lazy_static;
use parking_lot::Mutex;

pub use crate::error::{Result, SysxError};


lazy_static! {
    static ref ENV_VARS: Mutex<HashMap<String, String>> = Mutex::new(std::env::vars().collect());
}

pub fn set_env(key: &str, value: &str) -> Result<()> {
    unsafe {
        std::env::set_var(key, value);
    }
    ENV_VARS.lock().insert(key.to_string(), value.to_string());
    Ok(())
}

pub fn get_env(key: &str) -> Result<String> {
    std::env::var(key)
        .or_else(|_| ENV_VARS.lock()
            .get(key)
            .cloned()
            .ok_or_else(|| SysxError::AnyhowError(anyhow::anyhow!("Env var {} not found", key)))
        )
}

pub fn get_envs() -> HashMap<String, String> {
    ENV_VARS.lock().clone()
}

pub fn get_full_args() -> Vec<String> {
    std::env::args().collect()
}

pub fn get_args() -> Vec<String> {
    let mut args = get_full_args();
    let _ = args.remove(0);
    args
}

pub fn get_full_str_args() -> String {
    get_full_args().join(" ")
}

pub fn get_str_args() -> String {
    get_args().join(" ")
}
