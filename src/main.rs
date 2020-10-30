use std::io::{Error, ErrorKind};

use clap::{
  crate_authors, crate_description, crate_name, crate_version, 
  App, Arg
};

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
      Arg::with_name("ca_cert")
        .long("ca-certificate")
        .takes_value(true)
        .required(false)
        .requires("ca_pkey")
        .help("Path to ca certificate that should be used"),
    )
    .arg(
      Arg::with_name("ca_pkey")
        .long("ca-private-key")
        .takes_value(true)
        .required(false)
        .requires("ca_cert")
        .help("Path to ca private key that should be used"),
    )
    .get_matches();

  if matches.is_present("version") {
    println!(crate_version!());
    std::process::exit(0);
  };

  let settings = rusty_sailor::config::Settings::new(
    &matches.value_of("config")
  );

  let ca_pkey = matches.value_of("ca_pkey").map_or(
    Err(Error::new(ErrorKind::Other, "AAAA")),
    |ca_pkey_path| {
      rusty_sailor::pki::io::load_pem_private_key(ca_pkey_path)
    }
  );
  let ca_cert = matches.value_of("ca_cert").map_or(
    Err(Error::new(ErrorKind::Other, "BBB")),
    |ca_cert_path| {
      rusty_sailor::pki::io::load_pem_certificate(ca_cert_path)
    }
  );

  match settings {
    Ok(x) => {
      println!("Debug: {}", x.debug);
      println!("Pki.rsa_size: {}", x.pki.rsa_size);
      println!("Pki.country_name: {}", x.pki.country_name);
      println!("Pki.locality: {}", x.pki.locality);
      println!("Pki.organization: {}", x.pki.organization);
      println!("Pki.organizational_unit: {}", x.pki.organizational_unit);
      println!("Pki.state: {}", x.pki.state);
      println!("Pki.ca.common_name: {}", x.pki.ca.common_name);
      println!("Pki.ca.expiry_in_days: {}", x.pki.ca.expiry_in_days);
    }
    Err(y) => {
      println!("{}", y);
      std::process::exit(1);
    }
  };
}
