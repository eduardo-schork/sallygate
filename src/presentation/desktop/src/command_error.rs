use serde::Serialize;
use ts_rs::TS;

use crate::trust::HostKeyDto;

/// What went wrong in a command, in a form the UI can branch on and show.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(
    rename = "TCommandError",
    export,
    export_to = "../../ui/shared/command-error-contract.ts"
)]
pub struct CommandError {
    pub kind: CommandErrorKind,
    /// Safe to show the user; never contains secrets.
    pub message: String,
    /// The server key, for `unknownHostKey` and `hostKeyChanged`.
    #[ts(optional)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host_key: Option<HostKeyDto>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(
    rename = "TCommandErrorKind",
    export,
    export_to = "../../ui/shared/command-error-contract.ts"
)]
pub enum CommandErrorKind {
    InvalidInput,
    UnknownHostKey,
    HostKeyChanged,
    SecretRequired,
    AuthenticationRejected,
    KeyFileUnreadable,
    Unreachable,
    ConnectionFailed,
    KnownHostsUnavailable,
    HostStoreUnavailable,
    SshConfigUnreadable,
    SessionNotFound,
    SessionFailed,
    FilesUnavailable,
    FileOperationFailed,
    SnippetStoreUnavailable,
    KeyStoreUnavailable,
    TunnelStoreUnavailable,
    PortInUse,
    JumpHostFailed,
}

impl CommandError {
    pub fn new(kind: CommandErrorKind, message: &impl ToString) -> Self {
        Self {
            kind,
            message: message.to_string(),
            host_key: None,
        }
    }

    pub fn invalid_input(error: &impl ToString) -> Self {
        Self::new(CommandErrorKind::InvalidInput, error)
    }
}
