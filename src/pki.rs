use openssl::rsa::Rsa;

use crate::config;

pub fn create_ca_certificate(settings: config::PkiSettings) {
  let _ = Rsa::generate(settings.rsa_size);
}
