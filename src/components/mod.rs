use log::{error, info};

use crate::errors::InstallError;
use crate::install_ctx::InstallCtx;

pub mod ca;

pub type InstallStepResult = Result<InstallCtx, InstallError>;

// For some weird reason, having this function
// written as anonymous one, will make 
// compiler really unhappy.
// TODO: Place somewhere else
// TODO: Post issue
fn _bind(
  acc: InstallStepResult,
  f: &Fn(InstallCtx) -> InstallStepResult
) -> InstallStepResult {
  match acc {
    Ok(a) => f(a),
    Err(e) => Err(e)
  }
}

pub fn run_steps(
  install_ctx: InstallStepResult,
  install_components: Vec<&Fn(InstallCtx) -> InstallStepResult>
) -> Result<(), InstallError> {
  let status = match install_components.into_iter().fold(
    install_ctx,
    _bind
  ) {
    Ok(_) => {
      info!("Installation has been successfully completed!");
      Ok(())
    }
    Err(error) => {
      error!("Installation has failed!");
      error!("Error details: '{}'", error.to_string());
      Err(error)
    }
  };
  status
}
