use std::fs::create_dir_all;
use std::path::Path;

use crate::components::InstallStepResult;
use crate::errors::InstallError;
use crate::install_ctx::InstallCtx;
use crate::pki::cert::create_ca_certificate;
use crate::pki::io::{
  save_as_pem_certificate,
  save_as_pem_private_key
};

const CA_DIRNAME: &'static str = "pki";
const CA_PKEY_NAME: &'static str = "rusty-sailor-ca.private-key.pem";
const CA_CERT_NAME: &'static str = "rusty-sailor-ca.pem";

fn _load_custom_ca(
  mut ctx: InstallCtx,
  custom_ca_pkey_path: &Option<&str>,
  custom_ca_cert_path: &Option<&str>
) -> InstallStepResult {
  let ca_pkey = custom_ca_pkey_path.map_or(
    Err(InstallError::custom_ca_not_set()),
    |ca_pkey_path| {
      crate::pki::io::load_pem_private_key(ca_pkey_path)
    }
  );
  let ca_cert = custom_ca_cert_path.map_or(
    Err(InstallError::custom_ca_not_set()),
    |ca_cert_path| {
      crate::pki::io::load_pem_certificate(ca_cert_path)
    }
  );
  // TODO: Explicitly inform that custom ca was not found
  ctx.ca_private_key = ca_pkey.ok();
  ctx.ca_certificate = ca_cert.ok();
  Ok(ctx)
}

fn _ensure_ca_exists(
  mut ctx: InstallCtx,
) -> InstallStepResult {
  if ctx.ca_private_key.is_none() || ctx.ca_certificate.is_none() {
    let (ca_pkey, ca_cert) = create_ca_certificate(&ctx.config.pki)?;
    ctx.ca_private_key = Some(ca_pkey);
    ctx.ca_certificate = Some(ca_cert);
  }

  let target_dir = Path::new(
    &ctx.config.installation_dir
  ).join(CA_DIRNAME);

  create_dir_all(&target_dir)?;

  save_as_pem_private_key(
    &ctx.ca_private_key.as_ref().unwrap(),
    &target_dir.join(CA_PKEY_NAME)
  )?;
  save_as_pem_certificate(
    &ctx.ca_certificate.as_ref().unwrap(),
    &target_dir.join(CA_CERT_NAME)
  )?;

  Ok(ctx)
}

fn _ca_component(
  mut ctx: InstallCtx,
  custom_ca_pkey_path: &Option<&str>,
  custom_ca_cert_path: &Option<&str>
) -> InstallStepResult {
  _load_custom_ca(
    ctx,
    custom_ca_pkey_path,
    custom_ca_cert_path
  ).map_or_else(
    |e| Err(e),
    |context| _ensure_ca_exists(context)
  )
}

// http://blog.madhukaraphatak.com/functional-programming-in-rust-part-1/
// Returns function
pub fn ca_component<'a>(
  custom_ca_pkey_path:& 'a Option<&str>,
  custom_ca_cert_path:& 'a Option<&str>
) -> Box<dyn Fn(InstallCtx) -> InstallStepResult + 'a > {
  Box::new(move |ctx:InstallCtx| _ca_component(ctx, custom_ca_pkey_path, custom_ca_cert_path))
}
