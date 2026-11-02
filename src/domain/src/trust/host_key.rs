/// The public key a server presented during the handshake.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HostKey {
    algorithm: String,
    fingerprint: String,
    openssh_public_key: String,
}

impl HostKey {
    /// `fingerprint` is the SHA-256 form shown to users (`SHA256:…`), and
    /// `openssh_public_key` the single-line form stored in `known_hosts`.
    #[must_use]
    pub fn new(algorithm: String, fingerprint: String, openssh_public_key: String) -> Self {
        Self {
            algorithm,
            fingerprint,
            openssh_public_key,
        }
    }

    #[must_use]
    pub fn algorithm(&self) -> &str {
        &self.algorithm
    }

    #[must_use]
    pub fn fingerprint(&self) -> &str {
        &self.fingerprint
    }

    #[must_use]
    pub fn openssh_public_key(&self) -> &str {
        &self.openssh_public_key
    }
}
