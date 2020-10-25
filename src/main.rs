fn main() {
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

    let result = rusty_sailor::pki::create_ca_certificate(&x.pki);
    if let Ok((private_key, ca_cert)) = result {
      rusty_sailor::pki::save_as_pem_private_key(&private_key, &"ca.key.pem".to_string());
      rusty_sailor::pki::save_as_pem_certificate(&ca_cert, &"ca.pem".to_string());

      let second_result = rusty_sailor::pki::create_ca_signed_certificate(
        &x.pki,
        private_key,
        ca_cert,
        "blackwood".to_string(),
        13,
        Some(vec!["blackwood.local".to_string()]),
        Some(vec!["127.0.0.1".to_string()])
      );

      if let Ok((pkey, cert)) = second_result {
        rusty_sailor::pki::save_as_pem_private_key(&pkey, &"blackwood.key.pem".to_string());
        rusty_sailor::pki::save_as_pem_certificate(&cert, &"blackwood.pem".to_string());
      }
    }
  }
}
