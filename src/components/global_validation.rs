use std::net::IpAddr;

use crate::errors::{ErrorKind, InstallError};
use crate::components::InstallStepResult;
use crate::install_ctx::InstallCtx;

fn _bind_address_validation(
  bind_address: IpAddr
) -> Result<(), InstallError> {
  let is_valid = !bind_address.is_unspecified()
    && !bind_address.is_loopback()
    && !bind_address.is_multicast();
  if is_valid {
    return Ok(())
  }

  Err(
    InstallError::new(
      ErrorKind::BindAddress,
      format!("`{}` is not valid bind address", bind_address)
    )
  )
}

// DO NOT! USE WILLY-WILLY-NILLY
// ONLY IF IN-COMPONENT WILL NOT HELP
pub fn global_validation_component(
  mut install_ctx: InstallCtx
) -> InstallStepResult {
  _bind_address_validation(install_ctx.config.bind_address)?;
  Ok(install_ctx)
}
