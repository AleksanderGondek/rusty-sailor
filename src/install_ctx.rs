use openssl::pkey::{PKey, Private};
use openssl::x509::X509;

use crate::components::InstallStepResult;
use crate::config::Settings;
use crate::errors::InstallError;
use crate::logging::init_logger;

pub struct InstallCtx {
  pub ca_private_key: Option<PKey<Private>>,
  pub ca_certificate: Option<X509>,
  pub config: Settings
}

impl InstallCtx {
  pub fn new(
    custom_cfg_path: &Option<&str>
  ) -> Result<Self, InstallError> {
    let cfg = Settings::new(&custom_cfg_path)?;
    Ok(
      InstallCtx {
        ca_private_key: None,
        ca_certificate: None,
        config: cfg
      }
    )
  }

  pub fn new_with_init(
    custom_cfg_path: &Option<&str>
  ) -> InstallStepResult {
    let ctx = InstallCtx::new(custom_cfg_path)?;
    init_logger(&ctx.config)?;
    Ok(ctx)
  }
}
