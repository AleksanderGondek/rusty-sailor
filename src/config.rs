use config::{Config, ConfigError, File};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CaSettings {
  pub common_name: String,
  pub expiry_in_days: u32
}

#[derive(Debug, Deserialize)]
pub struct PkiSettings {
  // Shared x509 Attributes
  pub country_name: String,
  pub locality: String,
  pub organization: String,
  pub organizational_unit: String,
  pub state: String,
  pub email_address: String, 

  pub rsa_size: u32,
  pub ca: CaSettings
}

#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct Settings {
  pub debug: bool,
  pub installation_dir: String,
  pub pki: PkiSettings
}

impl Settings {
  pub fn new(filepath: &Option<&str>) -> Result<Self, ConfigError> {
    let mut cfg = Config::new();
    if let Some(filepath) = filepath {
      cfg.merge(File::with_name(filepath))?;
    }
    cfg.try_into()
  }
}

impl Default for Settings {
  fn default() -> Self {
    Settings {
      debug: true,
      installation_dir: "/tmp/rusty-sailor".to_string(),
      pki: PkiSettings {
        country_name: "PL".to_string(),
        locality: "Gdansk".to_string(),
        organization: "Rusty sailors ltd.".to_string(),
        organizational_unit: "R&D".to_string(),
        state: "Pomorskie".to_string(),
        email_address: "rust-sailor@k8s.eu".to_string(),
    
        rsa_size: 4096,
        ca: CaSettings {
          common_name: "rusty-sailor-ca".to_string(),
          expiry_in_days: 3650
        }
      }
    }
  }
}
