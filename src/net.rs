use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
use std::process::Command;

use crate::errors::{ErrorKind, InstallError};


pub fn guess_node_hostname() -> Option<String> {
  let output = Command::new("hostname").args(&["--fqdn"]).output().ok()?;
  let stdout = String::from_utf8(output.stdout).ok()?;
  Some(stdout)
}

pub fn guess_node_ip() -> Option<IpAddr> {
  let output = Command::new("hostname").args(&["-i"]).output().ok()?;
  let stdout = String::from_utf8(output.stdout).ok()?;

  let ip_addr_v4 = stdout.parse::<Ipv4Addr>();
  let ip_addr_v6 = stdout.parse::<Ipv6Addr>();

  if ip_addr_v4.is_ok() && ip_addr_v6.is_err() {
    return Some(IpAddr::V4(ip_addr_v4.ok()?));
  }
  if ip_addr_v4.is_err() && ip_addr_v6.is_ok() {
    return Some(IpAddr::V6(ip_addr_v6.ok()?));
  }
  None
}
