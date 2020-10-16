use config::{Config, ConfigError, File};
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct PkiSettings {
    pub test: String
}

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub debug: bool,
    pub pki: PkiSettings
}


impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let mut a = Config::new();
        a.merge(File::with_name("default.ini"))?;
        a.try_into()
    }
}
