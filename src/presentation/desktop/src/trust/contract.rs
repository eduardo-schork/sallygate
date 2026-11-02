use sallygate_domain::trust::HostKey;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::sessions::EndpointDto;

/// A server's public key as the UI shows it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(
    rename = "THostKey",
    export,
    export_to = "../../ui/trust/trust-contract.ts"
)]
pub struct HostKeyDto {
    pub algorithm: String,
    /// `SHA256:…`, the form users compare with the server's own output.
    pub fingerprint: String,
    pub openssh_public_key: String,
}

impl From<&HostKey> for HostKeyDto {
    fn from(key: &HostKey) -> Self {
        Self {
            algorithm: key.algorithm().to_owned(),
            fingerprint: key.fingerprint().to_owned(),
            openssh_public_key: key.openssh_public_key().to_owned(),
        }
    }
}

impl From<HostKeyDto> for HostKey {
    fn from(key: HostKeyDto) -> Self {
        Self::new(key.algorithm, key.fingerprint, key.openssh_public_key)
    }
}

/// The user decided to trust this key for this server.
#[derive(Debug, Clone, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(
    rename = "TTrustHostKeyRequest",
    export,
    export_to = "../../ui/trust/trust-contract.ts"
)]
pub struct TrustHostKeyRequest {
    pub endpoint: EndpointDto,
    pub host_key: HostKeyDto,
}
