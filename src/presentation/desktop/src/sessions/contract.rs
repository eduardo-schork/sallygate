use sallygate_domain::inventory::{HostAddress, Port, SshEndpoint, Username};
use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::command_error::CommandError;

/// Where to connect and as whom, as typed by the user.
#[derive(Debug, Clone, Deserialize, TS)]
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

/// Open a terminal on `endpoint`, logging in with the private key at `key_file_path`.
#[derive(Debug, Clone, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(
    rename = "TOpenSessionRequest",
    export,
    export_to = "../../ui/sessions/sessions-contract.ts"
)]
pub struct OpenSessionRequest {
    pub endpoint: EndpointDto,
    pub key_file_path: String,
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
