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
    .get_matches();

  if matches.is_present("version") {
    println!(crate_version!());
    std::process::exit(0);
  }

  let settings = rusty_sailor::config::Settings::new();

  println!("Hello, world!");
  if let Ok(x) = settings {
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
}
