use std::net::{IpAddr, Ipv4Addr};

use config::{Config, ConfigError, File};
use serde::Deserialize;

use crate::net::{guess_node_hostname, guess_node_ip};

#[derive(Debug, Deserialize)]
pub struct CaSettings {
  pub common_name: String,
  pub expiry_in_days: u32
}

#[derive(Debug, Deserialize)]
pub struct EtcdNode {
  pub name: String,
  pub peer_url: String
}

#[derive(Debug, Deserialize)]
pub struct EtcdSettings {
  pub data_dir: String,
  pub listen_peer_port: u32,
  pub listen_client_port: u32,
  pub other_nodes: Option<Vec<EtcdNode>>
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
  pub bind_address: IpAddr,
  pub debug: bool,  
  pub etcd: EtcdSettings,
  pub hostname: String,
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
      bind_address: guess_node_ip().unwrap_or(
        IpAddr::V4(Ipv4Addr::new(127,0,0,1))
      ),
      debug: false,
      etcd: EtcdSettings {
        data_dir: "/tmp/rusty-sailor/etcd/data".to_string(),
        listen_client_port: 2379,
        listen_peer_port: 2380,
        other_nodes: None
      },
      hostname: guess_node_hostname().unwrap_or(
        "localhost".to_string()
      ),
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
