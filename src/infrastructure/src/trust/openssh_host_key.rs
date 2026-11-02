use russh::keys::{HashAlg, PublicKey};
use sallygate_domain::trust::HostKey;

/// Describes a server's public key the way users and `known_hosts` see it.
///
/// # Errors
///
/// Returns the reason when the key cannot be written in `OpenSSH` form.
pub(crate) fn host_key_from_public_key(key: &PublicKey) -> Result<HostKey, String> {
    let openssh_public_key = key.to_openssh().map_err(|error| error.to_string())?;

    Ok(HostKey::new(
        key.algorithm().to_string(),
        key.fingerprint(HashAlg::Sha256).to_string(),
        openssh_public_key,
    ))
}

pub(crate) fn public_key_from_host_key(key: &HostKey) -> Result<PublicKey, String> {
    PublicKey::from_openssh(key.openssh_public_key()).map_err(|error| error.to_string())
}
