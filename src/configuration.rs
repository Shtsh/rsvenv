use config::{Config, ConfigError, Environment};
use lazy_static::lazy_static;
use serde_derive::Deserialize;
use std::sync::RwLock;

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct Settings {
    pub verbosity: i32,
    pub path: String,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let settings = Config::builder()
            .set_default("verbosity", 1)?
            .set_default("path", "~/.rsenv".to_string())?
            .add_source(Environment::with_prefix("RSVENV"))
            .build()?;
        settings.try_deserialize()
    }
}

lazy_static! {
    pub static ref SETTINGS: RwLock<Settings> = RwLock::new({ Settings::new().unwrap() });
}
