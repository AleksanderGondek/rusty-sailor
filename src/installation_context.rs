use std::io::{Error, ErrorKind};

use openssl::pkey::{PKey, Private};
use openssl::x509::X509;

use crate::config::Settings;

pub struct InstallationCtx {
  pub ca_certificate: Option<X509>,
  pub ca_private_key: Option<PKey<Private>>,
  pub config: Settings
}

impl InstallationCtx {
  pub fn new(
    custom_cfg_path: &Option<&str>
  ) -> Result<Self, Error> {
    let cfg_load_result = Settings::new(
      &custom_cfg_path
    );
    cfg_load_result.map_or_else(
      |e| Err(Error::new(ErrorKind::Other, e.to_string())),
      |cfg| Ok(
        InstallationCtx {
          ca_private_key: None,
          ca_certificate: None,
          config: cfg
        }
      )
    )
  }
}
