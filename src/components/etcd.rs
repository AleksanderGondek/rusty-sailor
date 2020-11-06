use crate::components::InstallStepResult;
use crate::install_ctx::InstallCtx;

pub fn etcd_component(
  mut install_ctx: InstallCtx
) -> InstallStepResult {
  Ok(install_ctx)
}
