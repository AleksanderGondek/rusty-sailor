use std::collections::HashSet;
use std::ffi::OsString;
use std::fs::create_dir_all;
use std::path::Path;

use crate::components::InstallStepResult;
use crate::fs::flatten;
use crate::install_ctx::InstallCtx;
use crate::vendored::unpack_archive;

const ETCD_DIRNAME: &'static str = "etcd";
const ETCD_ARCHIVE_NAME: &'static str = "etcd.tar.gz";

fn _get_etcd_files_to_extract() -> HashSet<OsString> {
  // In future, find a way to compile-time evaluate
  let mut etcd_artifacts_names = HashSet::new();
  etcd_artifacts_names.insert(OsString::from("etcd"));
  etcd_artifacts_names.insert(OsString::from("etcdctl"));
  let etcd_artifacts_names = etcd_artifacts_names;
  etcd_artifacts_names
}

pub fn etcd_component(
  mut install_ctx: InstallCtx
) -> InstallStepResult {
  let etcd_artifacts = _get_etcd_files_to_extract();
  let target_dir = Path::new(
    &install_ctx.config.installation_dir
  ).join(ETCD_DIRNAME);

  create_dir_all(&target_dir)?;
  unpack_archive(ETCD_ARCHIVE_NAME, &target_dir)?;
  flatten(&target_dir, Some(&etcd_artifacts))?;

  Ok(install_ctx)
}
