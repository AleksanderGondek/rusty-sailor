use openssl::pkey::{PKey, Private};
use openssl::x509::X509;

struct InstallationCtx {
  ca_certificate: X509,
  ca_private_key: PKey<Private>
}
