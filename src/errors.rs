use std::convert::From;
use std::error::Error;
use std::fmt;

#[derive(Debug, Clone)]
pub struct BaseError {
  pub msg: String
}

impl fmt::Display for BaseError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f,"{}",self.msg)
  }
}

impl From<openssl::error::ErrorStack> for BaseError {
  fn from(error: openssl::error::ErrorStack) -> Self {
    BaseError {
      msg: error.to_string()
    }
  }
}

impl From<std::io::Error> for BaseError {
  fn from(error: std::io::Error) -> Self {
    BaseError {
      msg: error.to_string()
    }
  }
}

impl From<config::ConfigError> for BaseError {
  fn from(error: config::ConfigError) -> Self {
    BaseError {
      msg: error.to_string()
    }
  }
}

impl From<log::SetLoggerError> for BaseError {
  fn from(error: log::SetLoggerError) -> Self {
    BaseError {
      msg: error.to_string()
    }
  }
}
