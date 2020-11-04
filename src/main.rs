use log::{error, info};
use clap::{
  crate_authors, crate_description, crate_name, crate_version, 
  App, Arg
};

use rusty_sailor::install_ctx::InstallCtx;
use rusty_sailor::logging::init_logger;
use rusty_sailor::prelude::InstallStepResult;

fn init(
  custom_cfg_path: &Option<&str>
) -> InstallStepResult {
  let ctx = InstallCtx::new(custom_cfg_path)?;
  init_logger(&ctx.config)?;
  Ok(ctx)
}

// For some weird reason, having this function
// written as anonymous one, will make 
// compiler really unhappy.
// TODO: Place somewhere else
// TODO: Post issue
fn _bind(
  acc: InstallStepResult,
  f: &Fn(InstallCtx) -> InstallStepResult
) -> InstallStepResult {
  match acc {
    Ok(a) => f(a),
    Err(e) => Err(e)
  }
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

  let ca_pkey_path = matches.value_of("ca_pkey");
  let ca_cert_path = matches.value_of("ca_cert");
  let create_ca_component = rusty_sailor::components::ca::ca_component(
    &ca_pkey_path,
    &ca_cert_path
  );

  let install_components: Vec<&Fn(InstallCtx) -> InstallStepResult> = vec![
    &create_ca_component
  ];

  let custom_config_path = matches.value_of("config");
  
  let install_ctx: InstallStepResult = init(&custom_config_path);
  match install_components.into_iter().fold(
    install_ctx,
    _bind
  ) {
    Ok(_) => {
      info!("Installation has been successfully completed!");
    }
    Err(error) => {
      error!("Installation has failed!");
      error!("Error details: '{}'", error.to_string());
    }
  }
}
