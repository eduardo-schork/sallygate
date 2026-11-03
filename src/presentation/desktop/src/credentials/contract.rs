use sallygate_domain::credentials::{Secret, SshKey};
use serde::{Deserialize, Serialize};
use ts_rs::TS;

/// A key pair in `~/.ssh` as the UI shows it. Holds no private material.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(
    rename = "TSshKey",
    export,
    export_to = "../../ui/credentials/credentials-contract.ts"
)]
pub struct SshKeyDto {
    pub name: String,
    /// `~/.ssh/<name>`, the form a host's key file field takes.
    pub path: String,
    pub algorithm: String,
    pub fingerprint: String,
    pub public_key: String,
    pub is_encrypted: bool,
}

impl From<&SshKey> for SshKeyDto {
    fn from(key: &SshKey) -> Self {
        Self {
            name: key.name.to_string(),
            path: key.path.clone(),
            algorithm: key.algorithm.clone(),
            fingerprint: key.fingerprint.clone(),
            public_key: key.public_key.clone(),
            is_encrypted: key.is_encrypted,
        }
    }
}

/// A new Ed25519 key. The passphrase, when typed, is used once and never kept (ADR-0015).
// No Debug: it would print the passphrase.
#[derive(Clone, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(
    rename = "TGenerateSshKeyRequest",
    export,
    export_to = "../../ui/credentials/credentials-contract.ts"
)]
pub struct GenerateSshKeyRequest {
    pub name: String,
    pub comment: String,
    #[ts(optional)]
    pub passphrase: Option<String>,
}

impl GenerateSshKeyRequest {
    /// Takes the passphrase out as a `Secret`; an empty field means no passphrase.
    pub fn take_passphrase(&mut self) -> Option<Secret> {
        self.passphrase
            .take()
            .filter(|passphrase| !passphrase.is_empty())
            .map(Secret::new)
    }
}
