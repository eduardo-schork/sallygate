use sallygate_domain::credentials::{AuthMethod, KeyFilePath, Secret};
use sallygate_domain::inventory::{HostAddress, Port, SshEndpoint, Username};
use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::command_error::CommandError;

/// Where to connect and as whom, as typed by the user.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(
    rename = "TEndpoint",
    export,
    export_to = "../../ui/sessions/sessions-contract.ts"
)]
pub struct EndpointDto {
    pub address: String,
    pub port: u16,
    pub username: String,
}

impl From<&SshEndpoint> for EndpointDto {
    fn from(endpoint: &SshEndpoint) -> Self {
        Self {
            address: endpoint.address().to_string(),
            port: endpoint.port().value(),
            username: endpoint.username().to_string(),
        }
    }
}

impl OpenSessionRequest {
    /// Takes the secret out, wrapped so it is wiped when dropped.
    pub fn take_secret(&mut self) -> Option<Secret> {
        self.secret.take().map(Secret::new)
    }
}

impl EndpointDto {
    pub fn into_domain(self) -> Result<SshEndpoint, CommandError> {
        Ok(SshEndpoint::new(
            HostAddress::parse(&self.address)
                .map_err(|error| CommandError::invalid_input(&error))?,
            Port::try_from(self.port).map_err(|error| CommandError::invalid_input(&error))?,
            Username::parse(&self.username).map_err(|error| CommandError::invalid_input(&error))?,
        ))
    }
}

/// How a host logs in. A password or passphrase is never part of it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(
    tag = "type",
    rename_all = "camelCase",
    rename_all_fields = "camelCase"
)]
#[ts(
    rename = "TAuth",
    export,
    export_to = "../../ui/sessions/sessions-contract.ts"
)]
pub enum AuthDto {
    KeyFile { path: String },
    Password,
}

impl From<&AuthMethod> for AuthDto {
    fn from(auth: &AuthMethod) -> Self {
        match auth {
            AuthMethod::KeyFile(path) => Self::KeyFile {
                path: path.as_path().display().to_string(),
            },
            AuthMethod::Password => Self::Password,
        }
    }
}

impl AuthDto {
    pub fn into_domain(self) -> Result<AuthMethod, CommandError> {
        Ok(match self {
            Self::KeyFile { path } => AuthMethod::KeyFile(
                KeyFilePath::parse(&path).map_err(|error| CommandError::invalid_input(&error))?,
            ),
            Self::Password => AuthMethod::Password,
        })
    }
}

/// Open a terminal on `endpoint`. `secret` is the password or key passphrase the user
/// just typed; it is used once and never saved (ADR-0015). No `Debug`, so it never prints.
#[derive(Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(
    rename = "TOpenSessionRequest",
    export,
    export_to = "../../ui/sessions/sessions-contract.ts"
)]
pub struct OpenSessionRequest {
    pub endpoint: EndpointDto,
    pub auth: AuthDto,
    #[ts(optional)]
    pub secret: Option<String>,
    pub columns: u16,
    pub rows: u16,
}

/// Everything a session tells the UI after it opens, in order.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, TS)]
#[serde(
    tag = "type",
    rename_all = "camelCase",
    rename_all_fields = "camelCase"
)]
#[ts(
    rename = "TSessionEvent",
    export,
    export_to = "../../ui/sessions/sessions-contract.ts"
)]
pub enum SessionEventDto {
    /// Bytes from the remote shell, base64-encoded.
    Output { data: String },
    /// The shell exited with this status.
    Exited { exit_status: u32 },
    /// The connection dropped.
    Disconnected { message: String },
}

#[cfg(test)]
mod tests {
    use super::*;

    fn to_json(event: &SessionEventDto) -> String {
        serde_json::to_string(event).unwrap()
    }

    #[test]
    fn session_events_are_tagged_by_type_with_camel_case_fields() {
        assert_eq!(
            to_json(&SessionEventDto::Output {
                data: "aGk=".to_owned()
            }),
            r#"{"type":"output","data":"aGk="}"#
        );
        assert_eq!(
            to_json(&SessionEventDto::Exited { exit_status: 3 }),
            r#"{"type":"exited","exitStatus":3}"#
        );
    }

    #[test]
    fn auth_is_tagged_by_type_and_round_trips() {
        let key_file = AuthDto::KeyFile {
            path: "~/.ssh/id_ed25519".to_owned(),
        };
        assert_eq!(
            serde_json::to_string(&key_file).unwrap(),
            r#"{"type":"keyFile","path":"~/.ssh/id_ed25519"}"#
        );
        assert_eq!(
            serde_json::to_string(&AuthDto::Password).unwrap(),
            r#"{"type":"password"}"#
        );
        let auth = key_file.into_domain().unwrap();
        assert_eq!(
            AuthDto::from(&auth),
            AuthDto::KeyFile {
                path: "~/.ssh/id_ed25519".to_owned()
            }
        );
    }

    #[test]
    fn rejects_an_endpoint_the_domain_does_not_accept() {
        let endpoint = EndpointDto {
            address: String::new(),
            port: 22,
            username: "ubuntu".to_owned(),
        };

        let error = endpoint.into_domain().unwrap_err();

        assert_eq!(
            error.kind,
            crate::command_error::CommandErrorKind::InvalidInput
        );
    }
}
