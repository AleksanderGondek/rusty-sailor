use clap::{
  crate_authors, crate_description, crate_name, crate_version, 
  App, Arg
};

use rusty_sailor::install_ctx::InstallCtx;

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

  let ca_pkey_path = matches.value_of("ca_pkey");
  let ca_cert_path = matches.value_of("ca_cert");
  let create_ca_component = rusty_sailor::components::ca::ca_component(
    &ca_pkey_path,
    &ca_cert_path
  );

  let _ = InstallCtx::new(
    &matches.value_of("config")
  ).map(
    |ctx| create_ca_component(ctx)
  );
}
