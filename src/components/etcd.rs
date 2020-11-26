use std::collections::HashSet;
use std::ffi::OsString;
use std::fs::create_dir_all;
use std::path::{Path, PathBuf};
use std::process::Command;

use askama::Template;

use crate::components::ca::{get_ca_cert_full_path};
use crate::components::InstallStepResult;
use crate::errors::{ErrorKind, InstallError};
use crate::fs::flatten;
use crate::install_ctx::InstallCtx;
use crate::pki::cert::create_ca_signed_certificate;
use crate::pki::io::{save_as_pem_private_key, save_as_pem_certificate};
use crate::templates::render_and_save;
use crate::vendored::unpack_archive;

const ETCD_DIRNAME: &'static str = "etcd";
const ETCD_ARCHIVE_NAME: &'static str = "etcd.tar.gz";
const ETCD_BINARY_NAME: &'static str = "etcd";
const ETCD_CERT_DIRNAME: &'static str = "certs";
const ETCD_CLIENT_PKEY_PATH: &'static str = "etcd-client.private-key.pem";
const ETCD_CLIENT_CERT_PATH: &'static str = "etcd-client.pem";
const ETCD_PEER_PKEY_PATH: &'static str = "etcd-peer.private-key.pem";
const ETCD_PEER_CERT_PATH: &'static str = "etcd-peer.pem";
const ETCD_CFG_FILE_NAME: &'static str = "etcd.conf.yml";
const ETCD_SYSTEMD_DEF_PATH: &'static str = "/etc/systemd/system/etcd.service";

#[derive(Template)]
#[template(path = "etcd/etcd.service", escape = "none")]
struct EtcdServiceTemplate<'a> {
  config_file_path: &'a str,
  exec_file_path: &'a str,
  installation_dir: &'a str
}

#[derive(Template)]
#[template(path = "etcd/etcd.conf.yml", escape = "none")]
struct EtcdConfigFileTemplate<'a> {
  member_name: &'a str,
  data_dir:  &'a str,
  listen_peer_urls: &'a String,
  listen_client_urls: &'a String,
  initial_cluster: &'a Vec<(String, String)>,
  cluster_token:  &'a str,
  ca_path:  &'a str,
  client_cert_path:  &'a str,
  client_cert_key_path:  &'a str,
  peer_cert_path:  &'a str,
  peer_cert_key_path:  &'a str,
}

fn _get_etcd_files_to_extract() -> HashSet<OsString> {
  // In future, find a way to compile-time evaluate
  let mut etcd_artifacts_names = HashSet::new();
  etcd_artifacts_names.insert(OsString::from("etcd"));
  etcd_artifacts_names.insert(OsString::from("etcdctl"));
  let etcd_artifacts_names = etcd_artifacts_names;
  etcd_artifacts_names
}

fn _get_etcd_paths(
  ctx: &InstallCtx
) -> (PathBuf,PathBuf,PathBuf,PathBuf,PathBuf,PathBuf,PathBuf,PathBuf,PathBuf) {
  let path_to_root_dir = Path::new(
    &ctx.config.installation_dir
  ).join(
    ETCD_DIRNAME
  );
  let path_to_data_dir = Path::new(
    &ctx.config.etcd.data_dir
  ).join(
    ETCD_DIRNAME
  );

  let path_to_certs_dir = path_to_root_dir.join(
    ETCD_CERT_DIRNAME
  );
  let path_to_binary = path_to_root_dir.join(
    ETCD_BINARY_NAME
  );
  let path_to_config_file = path_to_root_dir.join(
    ETCD_CFG_FILE_NAME
  );

  let path_to_client_pkey = path_to_certs_dir.join(
    ETCD_CLIENT_PKEY_PATH
  );
  let path_to_client_cert = path_to_certs_dir.join(
    ETCD_CLIENT_CERT_PATH
  );
  let path_to_peer_pkey = path_to_certs_dir.join(
    ETCD_PEER_PKEY_PATH
  );
  let path_to_peer_cert = path_to_certs_dir.join(
    ETCD_PEER_CERT_PATH
  );

  (
    path_to_root_dir,
    path_to_data_dir,
    path_to_certs_dir,
    path_to_binary,
    path_to_config_file,
    path_to_client_pkey,
    path_to_client_cert,
    path_to_peer_pkey,
    path_to_peer_cert
  )
}

fn _stringify(
  path: &Path
) -> Result<&str, InstallError> {
  path.to_str().map_or_else(
    || Err(InstallError::new(
      ErrorKind::Other,
      format!("Could not stringify `{:#?}`", &path)
    )),
    |x| Ok(x)
  )
}

fn _ensure_certificates_exit(
  install_ctx: &InstallCtx,
  path_to_peer_pkey: &Path,
  path_to_peer_cert: &Path,
  path_to_client_pkey: &Path,
  path_to_client_cert: &Path
) -> Result<(), InstallError> {
  let ca_private_key = install_ctx.ca_private_key.as_ref().map_or_else(
    || Err(InstallError::new_from_str(ErrorKind::Other, "CA private key not found in install_ctx")),
    |x| Ok(x)
  )?;
  let ca_certificate = install_ctx.ca_certificate.as_ref().map_or_else(
    || Err(InstallError::new_from_str(ErrorKind::Other, "CA cert not found in install_ctx")),
    |x| Ok(x)
  )?;
  
  let (peer_pkey, peer_cert) = create_ca_signed_certificate(
    &install_ctx.config.pki,
    &ca_private_key,
    &ca_certificate,
    &install_ctx.config.hostname,
    &365,
    &Some(vec![install_ctx.config.hostname.clone()]),
    &Some(vec![format!("{}", install_ctx.config.bind_address)])
  )?;
  save_as_pem_private_key(
    &peer_pkey,
    &path_to_peer_pkey
  )?;
  save_as_pem_certificate(
    &peer_cert,
    &path_to_peer_cert
  )?;

  let (client_pkey, client_cert) = create_ca_signed_certificate(
    &install_ctx.config.pki,
    &ca_private_key,
    &ca_certificate,
    &install_ctx.config.hostname,
    &365,
    &Some(vec![install_ctx.config.hostname.clone()]),
    &Some(vec![format!("{}", install_ctx.config.bind_address)])
  )?;
  save_as_pem_private_key(
    &client_pkey,
    &path_to_client_pkey
  )?;
  save_as_pem_certificate(
    &client_cert,
    &path_to_client_cert
  )?;

  Ok(())
}

fn _create_config_file(
  install_ctx: &InstallCtx,
  path_to_data_dir: &Path,
  path_to_config_file: &Path,
  path_to_ca_cert: &Path,
  path_to_peer_pkey: &Path,
  path_to_peer_cert: &Path,
  path_to_client_pkey: &Path,
  path_to_client_cert: &Path
) -> Result<(), InstallError> {
  let listen_peer_url = vec![
    format!(
      "https://{}:{}",
      install_ctx.config.bind_address,
      install_ctx.config.etcd.listen_peer_port
    )
  ];
  let listen_client_url = vec![
    format!(
      "https://{}:{}",
      install_ctx.config.bind_address,
      install_ctx.config.etcd.listen_client_port
    )
  ];
  let initial_cluster = vec![
    (
      install_ctx.config.hostname.clone(),
      format!(
        "https://{}:{}",
        install_ctx.config.bind_address,
        install_ctx.config.etcd.listen_peer_port
      )
    )
  ];

  render_and_save(
    EtcdConfigFileTemplate {
      member_name: &install_ctx.config.hostname,
      data_dir: &_stringify(&path_to_data_dir)?,
      listen_peer_urls: &listen_peer_url.join(", "),
      listen_client_urls: &listen_client_url.join(", "),
      initial_cluster: &initial_cluster,
      cluster_token: "etcd-cluster",
      ca_path: &_stringify(&path_to_ca_cert)?,
      client_cert_path: &_stringify(&path_to_client_cert)?,
      client_cert_key_path: &_stringify(&path_to_client_pkey)?,
      peer_cert_path: &_stringify(&path_to_peer_cert)?,
      peer_cert_key_path: &_stringify(&path_to_peer_pkey)?
    },
    &path_to_config_file
  )
}

fn _create_systemd_service_file(
  path_to_config_file: &Path,
  path_to_binary: &Path,
  path_to_root_dir: &Path
) -> Result<(), InstallError> {
  render_and_save(
    EtcdServiceTemplate {
      config_file_path: &_stringify(&path_to_config_file)?,
      exec_file_path: &_stringify(&path_to_binary)?,
      installation_dir: &_stringify(&path_to_root_dir)?
    },
    &Path::new(ETCD_SYSTEMD_DEF_PATH)
  )
}

fn _enable_systemd_service(
) -> Result<(), InstallError> {
  Command::new("sh")
    .arg("-c")
    .arg("systemctl reload")
    .output()?;
  Command::new("sh")
    .arg("-c")
    .arg("systemctl enable etcd.service")
    .output()?;
  Command::new("sh")
    .arg("-c")
    .arg("systemctl start etcd.service")
    .output()?;
  Ok(())
}

pub fn etcd_component(
  mut install_ctx: InstallCtx
) -> InstallStepResult {
  let etcd_artifacts = _get_etcd_files_to_extract();
  let path_to_ca_cert = get_ca_cert_full_path(&install_ctx);
  let (
    path_to_root_dir,
    path_to_data_dir,
    path_to_certs_dir,
    path_to_binary,
    path_to_config_file,
    path_to_client_pkey,
    path_to_client_cert,
    path_to_peer_pkey,
    path_to_peer_cert
  ) = _get_etcd_paths(&install_ctx);

  create_dir_all(&path_to_root_dir)?;
  create_dir_all(&path_to_data_dir)?;

  unpack_archive(ETCD_ARCHIVE_NAME, &path_to_root_dir)?;
  flatten(&path_to_root_dir, Some(&etcd_artifacts))?;

  create_dir_all(&path_to_certs_dir)?;
  _ensure_certificates_exit(
    &install_ctx,
    &path_to_client_pkey,
    &path_to_client_cert,
    &path_to_peer_pkey,
    &path_to_peer_cert
  )?;

  _create_config_file(
    &install_ctx,
    &path_to_data_dir,
    &path_to_config_file,
    &path_to_ca_cert,
    &path_to_client_pkey,
    &path_to_client_cert,
    &path_to_peer_pkey,
    &path_to_peer_cert
  )?;

  _create_systemd_service_file(
    &path_to_config_file,
    &path_to_binary,
    &path_to_root_dir
  )?;
  _enable_systemd_service()?;

  Ok(install_ctx)
}
