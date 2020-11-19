use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
use std::process::Command;

use crate::errors::{ErrorKind, InstallError};


pub fn guess_node_hostname() -> Result<String, InstallError> {
  let output = Command::new("hostname").args(&["--fqdn"]).output()?;
  let stdout = String::from_utf8(output.stdout)?;
  Ok(stdout)
}

pub fn guess_node_ip() -> Result<IpAddr, InstallError> {
  let output = Command::new("hostname").args(&["-i"]).output()?;
  let stdout = String::from_utf8(output.stdout)?;

  let ip_addr_v4 = stdout.parse::<Ipv4Addr>();
  let ip_addr_v6 = stdout.parse::<Ipv6Addr>();

  if ip_addr_v4.is_ok() && ip_addr_v6.is_err() {
    return Ok(IpAddr::V4(ip_addr_v4?));
  }
  if ip_addr_v4.is_err() && ip_addr_v6.is_ok() {
    return Ok(IpAddr::V6(ip_addr_v6?));
  }
  Err(
    InstallError::new_from_str(
      ErrorKind::Other,
      "Unable to parse `hostname -i` response"
    )
  )
}
