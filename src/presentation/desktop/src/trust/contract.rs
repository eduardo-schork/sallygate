use sallygate_domain::trust::{HostKey, KnownHostEntry};
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

/// One trusted key from `~/.ssh/known_hosts`. Sent back as is to forget it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(
    rename = "TKnownHost",
    export,
    export_to = "../../ui/trust/trust-contract.ts"
)]
pub struct KnownHostDto {
    /// The line in the file, from 1.
    pub line: u32,
    /// The servers as written; missing when the file stores them hashed.
    #[ts(optional)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hosts: Option<String>,
    pub host_key: HostKeyDto,
}

impl From<&KnownHostEntry> for KnownHostDto {
    fn from(entry: &KnownHostEntry) -> Self {
        Self {
            line: u32::try_from(entry.line).unwrap_or(u32::MAX),
            hosts: entry.hosts.clone(),
            host_key: HostKeyDto::from(&entry.host_key),
        }
    }
}

impl From<KnownHostDto> for KnownHostEntry {
    fn from(entry: KnownHostDto) -> Self {
        Self {
            line: usize::try_from(entry.line).unwrap_or(usize::MAX),
            hosts: entry.hosts,
            host_key: entry.host_key.into(),
        }
    }
}
