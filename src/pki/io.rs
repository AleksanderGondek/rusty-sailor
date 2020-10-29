use std::fs::File;
use std::io::Error;
use std::io::prelude::{Read, Write};

use openssl::pkey::{PKey, Private};
use openssl::x509::X509;

pub fn save_as_pem_private_key(
  key: &PKey<Private>,
  filename: &str
) -> Result<(), Error> {
  let mut file = File::create(filename)?;
  file.write_all(&key.private_key_to_pem_pkcs8()?)?;
  Ok(())
}

pub fn save_as_pem_certificate(
  certificate: &X509,
  filename: &str
) -> Result<(), Error> {
  let mut file = File::create(filename)?;
  file.write_all(&certificate.to_pem()?)?;
  Ok(())
}

pub fn load_pem_certificate(
  filepath: &str
) -> Result<X509, Error> {
  let mut file = File::open(filepath)?;
  let mut pem_bytes = Vec::new();
  
  file.read_to_end(&mut  pem_bytes)?;
  Ok(X509::from_pem(&pem_bytes)?)
}

pub fn load_pem_private_key(
  filepath: &str
) -> Result<PKey<Private>, Error> {
  let mut file = File::open(filepath)?;
  let mut pem_bytes = Vec::new();
  
  file.read_to_end(&mut  pem_bytes)?;
  Ok(PKey::private_key_from_pem(&pem_bytes)?)
}
