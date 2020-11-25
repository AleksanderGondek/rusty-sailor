use std::collections::HashSet;
use std::ffi::OsString;
use std::fs::create_dir_all;
use std::path::{Path, PathBuf};
use std::process::Command;

use askama::Template;

use crate::components::ca::{get_ca_key_full_path, get_ca_cert_full_path};
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

fn _get_etcd_root_dir(
  ctx: &InstallCtx
) -> PathBuf {
  Path::new(
    &ctx.config.installation_dir
  ).join(
    ETCD_DIRNAME
  )
}

fn _get_etcd_data_dir(
  ctx: &InstallCtx
) -> PathBuf {
  Path::new(
    &ctx.config.etcd.data_dir
  ).join(
    ETCD_DIRNAME
  )
}

fn _get_binary_full_path(
  ctx: &InstallCtx
) -> PathBuf {
  _get_etcd_root_dir(&ctx).join(
    ETCD_BINARY_NAME
  )
}

fn _get_config_full_path(
  ctx: &InstallCtx
) -> PathBuf {
  _get_etcd_root_dir(&ctx).join(
    ETCD_CFG_FILE_NAME
  )
}

fn _get_certs_dir_full_path(
  ctx: &InstallCtx
) -> PathBuf {
  _get_etcd_root_dir(&ctx).join(
    ETCD_CERT_DIRNAME
  )
}

fn _get_client_pkey_full_path(
  ctx: &InstallCtx
) -> PathBuf {
  _get_certs_dir_full_path(&ctx).join(ETCD_CLIENT_PKEY_PATH)
}

fn _get_client_cert_full_path(
  ctx: &InstallCtx
) -> PathBuf {
  _get_certs_dir_full_path(&ctx).join(ETCD_CLIENT_CERT_PATH)
}

fn _get_peer_pkey_full_path(
  ctx: &InstallCtx
) -> PathBuf {
  _get_certs_dir_full_path(&ctx).join(ETCD_PEER_PKEY_PATH)
}

fn _get_peer_cert_full_path(
  ctx: &InstallCtx
) -> PathBuf {
  _get_certs_dir_full_path(&ctx).join(ETCD_PEER_CERT_PATH)
}

fn _stringify(
  path: &PathBuf
) -> Result<&str, InstallError> {
  path.to_str().map_or_else(
    || Err(InstallError::new(
      ErrorKind::Other,
      format!("Could not stringify `{:#?}`", &path)
    )),
    |x| Ok(x)
  )
}

fn _ensure_certificates_exits(
  install_ctx: &InstallCtx
) -> Result<(), InstallError> {
  let ca_private_key = install_ctx.ca_private_key.as_ref().map_or_else(
    || Err(InstallError::new_from_str(ErrorKind::Other, "CA private key not found in install_ctx")),
    |x| Ok(x)
  )?;
  let ca_certificate = install_ctx.ca_certificate.as_ref().map_or_else(
    || Err(InstallError::new_from_str(ErrorKind::Other, "CA cert not found in install_ctx")),
    |x| Ok(x)
  )?;

  let cert_dir = _get_certs_dir_full_path(&install_ctx);
  create_dir_all(&cert_dir)?;

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
    &_get_peer_pkey_full_path(&install_ctx)
  )?;
  save_as_pem_certificate(
    &peer_cert,
    &_get_peer_cert_full_path(&install_ctx)
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
    &_get_client_pkey_full_path(&install_ctx)
  )?;
  save_as_pem_certificate(
    &client_cert,
    &_get_client_cert_full_path(&install_ctx)
  )?;

  Ok(())
}

fn _create_etcd_cfg_file(
  install_ctx: &InstallCtx,
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

  let path_to_config_file = _get_config_full_path(&install_ctx);
  let path_to_config_file = _stringify(&path_to_config_file)?;
  let path_to_ca_cert = get_ca_cert_full_path(&install_ctx);
  let path_to_ca_cert = _stringify(&path_to_ca_cert)?;
  let path_to_client_pkey = _get_client_pkey_full_path(&install_ctx);
  let path_to_client_pkey = _stringify(&path_to_client_pkey)?;
  let path_to_client_cert = _get_client_cert_full_path(&install_ctx);
  let path_to_client_cert = _stringify(&path_to_client_cert)?;
  let path_to_peer_pkey = _get_peer_pkey_full_path(&install_ctx);
  let path_to_peer_pkey = _stringify(&path_to_peer_pkey)?;
  let path_to_peer_cert = _get_peer_cert_full_path(&install_ctx);
  let path_to_peer_cert = _stringify(&path_to_peer_cert)?;

  render_and_save(
    EtcdConfigFileTemplate {
      member_name: &install_ctx.config.hostname,
      data_dir: &install_ctx.config.etcd.data_dir,
      listen_peer_urls: &listen_peer_url.join(", "),
      listen_client_urls: &listen_client_url.join(", "),
      initial_cluster: &initial_cluster,
      cluster_token: "etcd-cluster",
      ca_path: &path_to_ca_cert,
      client_cert_path: &path_to_client_cert,
      client_cert_key_path: &path_to_client_pkey,
      peer_cert_path: &path_to_peer_cert,
      peer_cert_key_path: &path_to_peer_pkey
    },
    &Path::new(path_to_config_file)
  )
}

fn _create_systemd_service_file(
  install_ctx: &InstallCtx,
) -> Result<(), InstallError> {
  let install_dir = _get_etcd_root_dir(&install_ctx);
  let install_dir = _stringify(&install_dir)?;
  let path_to_cfg_file = _get_config_full_path(&install_ctx);
  let path_to_cfg_file = _stringify(&path_to_cfg_file)?;
  let path_to_binary = _get_binary_full_path(&install_ctx);
  let path_to_binary = _stringify(&path_to_binary)?;

  render_and_save(
    EtcdServiceTemplate {
      config_file_path: path_to_cfg_file,
      exec_file_path: path_to_binary,
      installation_dir: install_dir
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
  
  let etcd_root_dir = _get_etcd_root_dir(&install_ctx);
  let etcd_data_dir = _get_etcd_data_dir(&install_ctx);

  create_dir_all(&etcd_root_dir)?;
  create_dir_all(&etcd_data_dir)?;

  unpack_archive(ETCD_ARCHIVE_NAME, &etcd_root_dir)?;
  flatten(&etcd_root_dir, Some(&etcd_artifacts))?;

  _ensure_certificates_exits(&install_ctx)?;
  _create_etcd_cfg_file(&install_ctx)?;

  _create_systemd_service_file(&install_ctx)?;
  _enable_systemd_service()?;

  Ok(install_ctx)
}
