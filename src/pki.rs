use std::fs::File;
use std::io::Error;
use std::io::prelude::Write;

use openssl::x509::{X509, X509Name};
use openssl::pkey::PKey;
use openssl::pkey::Private;
use openssl::hash::MessageDigest;
use openssl::rsa::Rsa;
use openssl::nid::Nid;
use openssl::error::ErrorStack;

use crate::config;

pub fn create_ca_certificate(
  settings: &config::PkiSettings
)-> Result<(PKey<Private>, X509), ErrorStack> {
  let pkey = PKey::from_rsa(Rsa::generate(settings.rsa_size)?)?;
  
  let mut ca_cn_builder = X509Name::builder()?;
  ca_cn_builder.append_entry_by_nid(Nid::COMMONNAME, &settings.ca.common_name)?;
  let name = ca_cn_builder.build();

  let mut builder = X509::builder()?;
  builder.set_version(2)?;
  builder.set_subject_name(&name)?;
  builder.set_issuer_name(&name)?;
  builder.set_pubkey(&pkey)?;
  builder.sign(&pkey, MessageDigest::sha256())?;

  let certificate: X509 = builder.build();
  Ok((pkey, certificate))
}

pub fn save_as_pem_private_key(key: PKey<Private>) -> Result<(), Error> {
  let mut file = File::create("test2.pem")?;
  file.write_all(&key.private_key_to_pem_pkcs8()?)?;
  Ok(())
}

pub fn save_as_pem_certificate(certificate: X509) -> Result<(), Error> {
  let mut file = File::create("test.pem")?;
  file.write_all(&certificate.to_pem()?)?;
  Ok(())
}
