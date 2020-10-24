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

      let result = rusty_sailor::pki::create_ca_certificate(&x.pki);
      match result {
        Ok((private_key, ca_cert)) => {
          rusty_sailor::pki::save_as_pem_private_key(private_key);
          rusty_sailor::pki::save_as_pem_certificate(ca_cert);
        }
        Err(_) => {
          println!("Failed to create ca certificate!");
        }
      }
    },
    Err(_) => {},
  }
}
