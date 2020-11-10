use std::collections::HashSet;
use std::ffi::OsString;
use std::fs::create_dir_all;
use std::path::Path;

use askama::Template;

use crate::components::InstallStepResult;
use crate::errors::{ErrorKind, InstallError};
use crate::fs::flatten;
use crate::install_ctx::InstallCtx;
use crate::templates::render_and_save;
use crate::vendored::unpack_archive;

const ETCD_DIRNAME: &'static str = "etcd";
const ETCD_ARCHIVE_NAME: &'static str = "etcd.tar.gz";
const ETCD_BINARY_NAME: &'static str = "etcd";
const ETCD_ENV_FILE_NAME: &'static str = "etcd.env";

#[derive(Template)]
#[template(path = "etcd/etcd.service", escape = "none")]
struct EtcdServiceTemplate<'a> {
  env_file_path: &'a str,
  exec_file_path: &'a str,
  installation_dir: &'a str
}

fn _get_etcd_files_to_extract() -> HashSet<OsString> {
  // In future, find a way to compile-time evaluate
  let mut etcd_artifacts_names = HashSet::new();
  etcd_artifacts_names.insert(OsString::from("etcd"));
  etcd_artifacts_names.insert(OsString::from("etcdctl"));
  let etcd_artifacts_names = etcd_artifacts_names;
  etcd_artifacts_names
}

fn _create_systemd_service_file(
  target_dir: &Path,
) -> Result<(), InstallError> {
  let path_to_env_file = target_dir.join(
    ETCD_ENV_FILE_NAME
  );
  let path_to_binary = target_dir.join(
    ETCD_BINARY_NAME
  );

  let path_to_env_file = path_to_env_file.to_str().map_or_else(
    || Err(InstallError::new_from_str(ErrorKind::Other, "??")),
    |x| Ok(x)
  )?;
  let path_to_binary = path_to_binary.to_str().map_or_else(
    || Err(InstallError::new_from_str(ErrorKind::Other, "??")),
    |x| Ok(x)
  )?;
  let install_dir = target_dir.to_str().map_or_else(
    || Err(InstallError::new_from_str(ErrorKind::Other, "??")),
    |x| Ok(x)
  )?;

  render_and_save(
    EtcdServiceTemplate {
      env_file_path: path_to_env_file,
      exec_file_path: path_to_binary,
      installation_dir: install_dir
    },
    // TODO: Change to proper value later
    &Path::new("/tmp/rusty-sailor/etcd/etcd.service")
  )
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

  _create_systemd_service_file(&target_dir)?;

  Ok(install_ctx)
}
