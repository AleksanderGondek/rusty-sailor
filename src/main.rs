extern crate rusty_sailor;

fn main() {
  let settings = rusty_sailor::config::Settings::new();

  println!("Hello, world!");
  match settings {
    Ok(x) => {
      println!("Debug: {}", x.debug);
      println!("Pki.rsa_size: {}", x.pki.rsa_size);
      println!("Pki.ca.common_name: {}", x.pki.ca.common_name);
      println!("Pki.ca.country_name: {}", x.pki.ca.country_name);
      println!("Pki.ca.locality: {}", x.pki.ca.locality);
      println!("Pki.ca.organization: {}", x.pki.ca.organization);
      println!("Pki.ca.organizational_unit: {}", x.pki.ca.organizational_unit);
      println!("Pki.ca.state: {}", x.pki.ca.state);
      println!("Pki.ca.expiry_in_days: {}", x.pki.ca.expiry_in_days);
    },
    Err(_) => {},
  }
}
