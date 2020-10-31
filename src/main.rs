use std::io::{Error, ErrorKind};

use clap::{
  crate_authors, crate_description, crate_name, crate_version, 
  App, Arg
};

use rusty_sailor::installation_context::InstallationCtx;

fn _load_custom_ca(
  mut ctx: InstallationCtx,
  custom_ca_pkey_path: &Option<&str>,
  custom_ca_cert_path: &Option<&str>
) -> Result<InstallationCtx, Error> {
  let ca_pkey = custom_ca_pkey_path.map_or(
    Err(Error::new(ErrorKind::Other, "Custom CA private key not provided!")),
    |ca_pkey_path| {
      rusty_sailor::pki::io::load_pem_private_key(ca_pkey_path)
    }
  );
  let ca_cert = custom_ca_cert_path.map_or(
    Err(Error::new(ErrorKind::Other, "Custom CA certificate not provided")),
    |ca_cert_path| {
      rusty_sailor::pki::io::load_pem_certificate(ca_cert_path)
    }
  );
  // TODO: Explicitly inform that custom ca was not found
  ctx.ca_private_key = ca_pkey.ok();
  ctx.ca_certificate = ca_cert.ok();
  Ok(ctx)
}

fn main() {
  let matches = App::new(crate_name!())
    .version(crate_version!())
    .author(crate_authors!())
    .about(crate_description!())
    .arg(
      Arg::with_name("version")
        .long("version")
        .takes_value(false)
        .help("Prints current rusty-sailor version"),
    )
    .arg(
      Arg::with_name("config")
        .long("config")
        .short("c")
        .takes_value(true)
        .required(false)
        .help("Path to configuration file which should be used"),
    )
    .arg(
      Arg::with_name("ca_pkey")
        .long("ca-private-key")
        .takes_value(true)
        .required(false)
        .requires("ca_cert")
        .help("Path to ca private key that should be used"),
    )
    .arg(
      Arg::with_name("ca_cert")
        .long("ca-certificate")
        .takes_value(true)
        .required(false)
        .requires("ca_pkey")
        .help("Path to ca certificate that should be used"),
    )
    .get_matches();

  if matches.is_present("version") {
    println!(crate_version!());
    std::process::exit(0);
  };

  let custom_ca_pkey_path = matches.value_of("ca_pkey");
  let custom_ca_cert_path = matches.value_of("ca_cert");

  let test = InstallationCtx::new(&matches.value_of("config"))
    .map(|ctx| {
      _load_custom_ca(
        ctx, 
        &custom_ca_pkey_path,
        &custom_ca_cert_path
      )
    });
}
