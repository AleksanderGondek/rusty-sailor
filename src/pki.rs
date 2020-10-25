use std::fs::File;
use std::io::Error;
use std::io::prelude::Write;

use openssl::asn1::Asn1Time;
use openssl::bn::{BigNum, MsbOption};
use openssl::x509::{extension, X509, X509Name, X509Req};
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
    Nid::COUNTRYNAME, &settings.country_name
  )?;
  ca_name.append_entry_by_nid(
    Nid::LOCALITYNAME, &settings.locality
  )?;
  ca_name.append_entry_by_nid(
    Nid::ORGANIZATIONNAME, &settings.organization
  )?;
  ca_name.append_entry_by_nid(
    Nid::ORGANIZATIONALUNITNAME, &settings.organizational_unit
  )?;
  ca_name.append_entry_by_nid(
    Nid::STATEORPROVINCENAME, &settings.state
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
    .issuer(true)
    .build(&ca_cert.x509v3_context(None, None))?;
  ca_cert.append_extension(authority_key_identifier)?;

  ca_cert.sign(&private_key, MessageDigest::sha256())?;
  let ca_cert: X509 = ca_cert.build();
  Ok((private_key, ca_cert))
}

pub fn create_csr(
  settings: &config::PkiSettings,
  private_key: &PKey<Private>,
  common_name: String
) -> Result<X509Req, ErrorStack> {
  let mut csr = openssl::x509::X509ReqBuilder::new()?;
  csr.set_pubkey(&private_key)?;

  let mut name = X509Name::builder()?;
  name.append_entry_by_nid(
    Nid::COMMONNAME, &common_name
  )?;
  name.append_entry_by_nid(
    Nid::COUNTRYNAME, &settings.country_name
  )?;
  name.append_entry_by_nid(
    Nid::LOCALITYNAME, &settings.locality
  )?;
  name.append_entry_by_nid(
    Nid::ORGANIZATIONNAME, &settings.organization
  )?;
  name.append_entry_by_nid(
    Nid::ORGANIZATIONALUNITNAME, &settings.organizational_unit
  )?;
  name.append_entry_by_nid(
    Nid::STATEORPROVINCENAME, &settings.state
  )?;
  let name = name.build();

  csr.set_subject_name(&name)?;
  csr.sign(&private_key, MessageDigest::sha256())?;
  let csr = csr.build();
  Ok(csr)
}

pub fn create_ca_signed_certificate(
  settings: &config::PkiSettings,
  ca_private_key: PKey<Private>,
  ca_cert: X509,
  common_name: String,
  expiry_in_days: u32,
  alt_names_dns: Option<Vec<String>>,
  alt_names_ip: Option<Vec<String>>
) -> Result<(PKey<Private>, X509), ErrorStack> {
  let private_key = PKey::from_rsa(
    Rsa::generate(settings.rsa_size)?
  )?;

  let csr = create_csr(
    &settings,
    &private_key,
    common_name
  )?;

  // There has to be a better way
  let serial_number = {
    let mut serial = BigNum::new()?;
    serial.rand(159, MsbOption::MAYBE_ZERO, false)?;
    serial.to_asn1_integer()?
  };

  let mut cert = X509::builder()?;
  cert.set_version(2)?;
  cert.set_serial_number(&serial_number)?;
  cert.set_pubkey(&private_key)?;
  cert.set_subject_name(csr.subject_name())?;
  cert.set_issuer_name(ca_cert.subject_name())?;

  let not_before = Asn1Time::days_from_now(0)?;
  cert.set_not_before(&not_before)?;
  let not_after = Asn1Time::days_from_now(
    expiry_in_days
  )?;
  cert.set_not_after(&not_after)?;

  cert.append_extension(
    extension::BasicConstraints::new().build()?
  )?;

  cert.append_extension(
    extension::KeyUsage::new()
    .critical()
    .non_repudiation()
    .digital_signature()
    .key_encipherment()
    .build()?
  )?;

  let subject_key_identifier = extension::SubjectKeyIdentifier::new()
    .build(&cert.x509v3_context(Some(&ca_cert), None))?;
  cert.append_extension(subject_key_identifier)?;

  let authority_key_identifier = extension::AuthorityKeyIdentifier::new()
    .keyid(true)
    .issuer(true)
    .build(&cert.x509v3_context(Some(&ca_cert), None))?;
  cert.append_extension(authority_key_identifier)?;

  let mut san = extension::SubjectAlternativeName::new();
  if let Some(alt_names) = alt_names_dns {
    alt_names.iter().for_each(|name| { san.dns(name); });
  }
  if let Some(alt_names) = alt_names_ip {
    alt_names.iter().for_each(|ip| { san.ip(ip); });
  }
  let san = san.build(
    &cert.x509v3_context(Some(&ca_cert), None)
  )?;
  cert.append_extension(san)?;

  cert.sign(&ca_private_key, MessageDigest::sha256())?;
  let cert = cert.build();
  Ok((private_key, cert))
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
