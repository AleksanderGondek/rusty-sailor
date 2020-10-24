use config::{Config, ConfigError, File};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CaSettings {
    // x509 Attributes
    pub common_name: String,
    pub country_name: String,
    pub locality: String,
    pub organization: String,
    pub organizational_unit: String,
    pub state: String, 

    pub expiry_in_days: u32
}

#[derive(Debug, Deserialize)]
pub struct PkiSettings {
    pub rsa_size: u32,
    pub ca: CaSettings
}

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub debug: bool,
    pub pki: PkiSettings
}


impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let mut a = Config::new();
        a.merge(File::with_name("default.toml"))?;
        a.try_into()
    }
}
