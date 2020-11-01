use std::fs::create_dir_all;
use std::io::{Error, ErrorKind};
use std::path::Path;

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
  if ctx.ca_private_key.is_none() || ctx.ca_certificate.is_none() {
    let (ca_pkey, ca_cert) = crate::pki::cert::create_ca_certificate(&ctx.config.pki)?;
    ctx.ca_private_key = Some(ca_pkey);
    ctx.ca_certificate = Some(ca_cert);
  }

  let target_dir = Path::new(
    &ctx.config.installation_dir
  ).join(
    Path::new("pki")
  );

  create_dir_all(&target_dir)?;

  crate::pki::io::save_as_pem_private_key(
    &ctx.ca_private_key.as_ref().unwrap(),
    &target_dir.join(Path::new("rusty-sailor-ca.private-key.pem"))
  )?;
  crate::pki::io::save_as_pem_certificate(
    &ctx.ca_certificate.as_ref().unwrap(),
    &target_dir.join(Path::new("rusty-sailor-ca.pem"))
  )?;

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
