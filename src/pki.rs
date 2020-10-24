use std::fs::File;
use std::io::Error;
use std::io::prelude::Write;

use openssl::asn1::Asn1Time;
use openssl::bn::{BigNum, MsbOption};
use openssl::x509::{extension, X509, X509Name};
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
  let private_key = PKey::from_rsa(
    Rsa::generate(settings.rsa_size)?
  )?;

  let mut ca_name = X509Name::builder()?;
  ca_name.append_entry_by_nid(
    Nid::COMMONNAME, &settings.ca.common_name
  )?;
  ca_name.append_entry_by_nid(
    Nid::COUNTRYNAME, &settings.ca.country_name
  )?;
  ca_name.append_entry_by_nid(
    Nid::LOCALITYNAME, &settings.ca.locality
  )?;
  ca_name.append_entry_by_nid(
    Nid::ORGANIZATIONNAME, &settings.ca.organization
  )?;
  ca_name.append_entry_by_nid(
    Nid::ORGANIZATIONALUNITNAME, &settings.ca.organizational_unit
  )?;
  ca_name.append_entry_by_nid(
    Nid::STATEORPROVINCENAME, &settings.ca.state
  )?;
  let ca_name = ca_name.build();

  // There has to be a better way
  let serial_number = {
    let mut serial = BigNum::new()?;
    serial.rand(159, MsbOption::MAYBE_ZERO, false)?;
    serial.to_asn1_integer()?
  };

  let mut ca_cert = X509::builder()?;
  ca_cert.set_version(2)?;
  ca_cert.set_serial_number(&serial_number)?;
  ca_cert.set_subject_name(&ca_name)?;
  ca_cert.set_issuer_name(&ca_name)?;
  ca_cert.set_pubkey(&private_key)?;

  let not_before = Asn1Time::days_from_now(0)?;
  ca_cert.set_not_before(&not_before)?;
  let not_after = Asn1Time::days_from_now(
    settings.ca.expiry_in_days
  )?;
  ca_cert.set_not_after(&not_after)?;

  ca_cert.append_extension(
    extension::BasicConstraints::new()
    .critical()
    .ca()
    .build()?
  )?;

  ca_cert.append_extension(
    extension::KeyUsage::new()
    .critical()
    .key_cert_sign()
    .crl_sign()
    .build()?
  )?;

  let subject_key_identifier = extension::SubjectKeyIdentifier::new()
    .build(&ca_cert.x509v3_context(None, None))?;
  ca_cert.append_extension(subject_key_identifier)?;

  let authority_key_identifier = extension::AuthorityKeyIdentifier::new()
    .keyid(true)
    .build(&ca_cert.x509v3_context(None, None))?;
  ca_cert.append_extension(authority_key_identifier)?;

  ca_cert.sign(&private_key, MessageDigest::sha256())?;
  let ca_cert: X509 = ca_cert.build();
  Ok((private_key, ca_cert))
}

pub fn save_as_pem_private_key(key: PKey<Private>) -> Result<(), Error> {
  let mut file = File::create("test.private-key.pem")?;
  file.write_all(&key.private_key_to_pem_pkcs8()?)?;
  Ok(())
}

pub fn save_as_pem_certificate(certificate: X509) -> Result<(), Error> {
  let mut file = File::create("test.pem")?;
  file.write_all(&certificate.to_pem()?)?;
  Ok(())
}
