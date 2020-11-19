use std::convert::From;
use std::fmt;

#[derive(Debug, Clone)]
pub enum ErrorKind {
  Config,
  CustomCANotSet,
  FileIo,
  Logger,
  OpenSSL,
  TemplateRender,
  UnpackArchive,
  Other
}

#[derive(Debug, Clone)]
pub struct InstallError {
  pub kind: ErrorKind,
  pub msg: String
}

impl InstallError {
  pub fn new(
    kind: ErrorKind,
    msg: String
  ) -> Self {
    Self {
      kind,
      msg
    }
  }

  pub fn new_from_str(
    kind: ErrorKind,
    msg: &str
  ) -> Self {
    Self {
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

impl fmt::Display for InstallError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f,"{}",self.msg)
  }
}

impl From<openssl::error::ErrorStack> for InstallError {
  fn from(error: openssl::error::ErrorStack) -> Self {
    InstallError::new(ErrorKind::OpenSSL, error.to_string())
  }
}

impl From<std::io::Error> for InstallError {
  fn from(error: std::io::Error) -> Self {
    InstallError::new(ErrorKind::FileIo, error.to_string())
  }
}

impl From<config::ConfigError> for InstallError {
  fn from(error: config::ConfigError) -> Self {
    InstallError::new(ErrorKind::Config, error.to_string())
  }
}

impl From<log::SetLoggerError> for InstallError {
  fn from(error: log::SetLoggerError) -> Self {
    InstallError::new(ErrorKind::Logger, error.to_string())
  }
}

impl From<askama::shared::Error> for InstallError {
  fn from(error: askama::shared::Error) -> Self {
    InstallError::new(ErrorKind::TemplateRender, error.to_string())
  }
}

impl From<std::string::FromUtf8Error> for InstallError {
  fn from(error: std::string::FromUtf8Error) -> Self {
    InstallError::new(ErrorKind::Config, error.to_string())
  }
}

impl From<std::net::AddrParseError> for InstallError {
  fn from(error: std::net::AddrParseError) -> Self {
    InstallError::new(ErrorKind::Config, error.to_string())
  }
}