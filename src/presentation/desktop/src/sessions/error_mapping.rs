use sallygate_application::sessions::{OpenSessionError, SessionCommandError};

use crate::command_error::{CommandError, CommandErrorKind};
use crate::trust::HostKeyDto;

pub fn from_open_session_error(error: OpenSessionError) -> CommandError {
    let message = error.to_string();
    let (kind, host_key) = match error {
        OpenSessionError::UnknownHostKey(key) => (CommandErrorKind::UnknownHostKey, Some(key)),
        OpenSessionError::HostKeyChanged(key) => (CommandErrorKind::HostKeyChanged, Some(key)),
        OpenSessionError::AuthenticationRejected => {
            (CommandErrorKind::AuthenticationRejected, None)
        }
        OpenSessionError::KeyFileUnreadable(_) => (CommandErrorKind::KeyFileUnreadable, None),
        OpenSessionError::Unreachable(_) => (CommandErrorKind::Unreachable, None),
        OpenSessionError::Protocol(_) => (CommandErrorKind::ConnectionFailed, None),
        OpenSessionError::KnownHosts(_) => (CommandErrorKind::KnownHostsUnavailable, None),
    };

    CommandError {
        kind,
        message,
        host_key: host_key.as_ref().map(HostKeyDto::from),
    }
}

pub fn from_session_command_error(error: &SessionCommandError) -> CommandError {
    let kind = match error {
        SessionCommandError::NotFound(_) => CommandErrorKind::SessionNotFound,
        SessionCommandError::Session(_) => CommandErrorKind::SessionFailed,
    };
    CommandError::new(kind, error)
}

#[cfg(test)]
mod tests {
    use sallygate_domain::trust::HostKey;

    use super::*;

    #[test]
    fn an_unknown_server_key_reaches_the_ui_with_its_fingerprint() {
        let key = HostKey::new(
            "ssh-ed25519".to_owned(),
            "SHA256:abc".to_owned(),
            "ssh-ed25519 AAAA".to_owned(),
        );

        let error = from_open_session_error(OpenSessionError::UnknownHostKey(key));

        assert_eq!(error.kind, CommandErrorKind::UnknownHostKey);
        assert_eq!(
            error.host_key.map(|host_key| host_key.fingerprint),
            Some("SHA256:abc".to_owned())
        );
    }

    #[test]
    fn other_failures_carry_no_host_key() {
        let error = from_open_session_error(OpenSessionError::AuthenticationRejected);

        assert_eq!(error.kind, CommandErrorKind::AuthenticationRejected);
        assert_eq!(error.host_key, None);
    }
}
