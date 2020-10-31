use std::io::{Error, ErrorKind};

use crate::installation_context::InstallationCtx;

fn _load_custom_ca(
  mut ctx: InstallationCtx,
  custom_ca_pkey_path: &Option<&str>,
  custom_ca_cert_path: &Option<&str>
) -> Result<InstallationCtx, Error> {
  let ca_pkey = custom_ca_pkey_path.map_or(
    Err(Error::new(ErrorKind::Other, "Custom CA private key not provided!")),
    |ca_pkey_path| {
      crate::pki::io::load_pem_private_key(ca_pkey_path)
    }
  );
  let ca_cert = custom_ca_cert_path.map_or(
    Err(Error::new(ErrorKind::Other, "Custom CA certificate not provided")),
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
  mut ctx: InstallationCtx,
) -> Result<InstallationCtx, Error> {
  if ctx.ca_private_key.is_some() && ctx.ca_certificate.is_some() {
    return Ok(ctx)
  }
  Ok(ctx)
}

fn _ca_component(
  mut ctx: InstallationCtx,
  custom_ca_pkey_path: &Option<&str>,
  custom_ca_cert_path: &Option<&str>
) -> Result<InstallationCtx, Error> {
  _load_custom_ca(
    ctx,
    custom_ca_pkey_path,
    custom_ca_cert_path
  ).map_or_else(
    |e| Err(Error::new(ErrorKind::Other, e.to_string())),
    |context| _ensure_ca_exists(context)
  )
}

// http://blog.madhukaraphatak.com/functional-programming-in-rust-part-1/
// Returns function
pub fn ca_component<'a>(
  custom_ca_pkey_path:& 'a Option<&str>,
  custom_ca_cert_path:& 'a Option<&str>
) -> Box<Fn(InstallationCtx) -> Result<InstallationCtx, Error> + 'a > {
  Box::new(move |ctx:InstallationCtx| _ca_component(ctx, custom_ca_pkey_path, custom_ca_cert_path))
}
