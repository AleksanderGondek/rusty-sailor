use std::convert::From;
use std::fmt;

#[derive(Debug, Clone)]
pub enum ErrorKind {
  Config,
  CustomCANotSet,
  FileIo,
  Logger,
  OpenSSL,
  Other
}

#[derive(Debug, Clone)]
pub struct BaseError {
  pub kind: ErrorKind,
  pub msg: String
}

impl BaseError {
  pub fn new(
    kind: ErrorKind,
    msg: String
  ) -> Self {
    BaseError {
      kind,
      msg
    }
  }

  pub fn new_from_str(
    kind: ErrorKind,
    msg: &str
  ) -> Self {
    BaseError {
      kind,
      msg: msg.to_string()
    }
  }

  pub fn custom_ca_not_set() -> Self {
    Self::new_from_str(
      ErrorKind::CustomCANotSet,
      "Custom CA was not provided for the installer run."
    )
  }
}

impl fmt::Display for BaseError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f,"{}",self.msg)
  }
}

impl From<openssl::error::ErrorStack> for BaseError {
  fn from(error: openssl::error::ErrorStack) -> Self {
    BaseError::new(ErrorKind::OpenSSL, error.to_string())
  }
}

impl From<std::io::Error> for BaseError {
  fn from(error: std::io::Error) -> Self {
    BaseError::new(ErrorKind::FileIo, error.to_string())
  }
}

impl From<config::ConfigError> for BaseError {
  fn from(error: config::ConfigError) -> Self {
    BaseError::new(ErrorKind::Config, error.to_string())
  }
}

impl From<log::SetLoggerError> for BaseError {
  fn from(error: log::SetLoggerError) -> Self {
    BaseError::new(ErrorKind::Logger, error.to_string())
  }
}
