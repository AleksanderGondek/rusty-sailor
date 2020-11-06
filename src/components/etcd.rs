use std::fs::create_dir_all;
use std::path::Path;

use crate::components::InstallStepResult;
use crate::install_ctx::InstallCtx;

const ETCD_DIRNAME: &'static str = "etcd";

pub fn etcd_component(
  mut install_ctx: InstallCtx
) -> InstallStepResult {
  let target_dir = Path::new(
    &install_ctx.config.installation_dir
  ).join(ETCD_DIRNAME);

  create_dir_all(&target_dir)?;

  Ok(install_ctx)
}
