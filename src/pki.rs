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
