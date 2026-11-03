use sallygate_application::inventory::ImportReport;
use sallygate_domain::inventory::{GroupName, Host, HostId, HostLabel};
use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::command_error::CommandError;
use crate::sessions::{AuthDto, EndpointDto};

/// A saved host as the UI shows it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(
    rename = "THost",
    export,
    export_to = "../../ui/inventory/inventory-contract.ts"
)]
pub struct HostDto {
    pub id: String,
    pub label: String,
    pub endpoint: EndpointDto,
    pub auth: AuthDto,
    #[ts(optional)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
}

impl From<&Host> for HostDto {
    fn from(host: &Host) -> Self {
        Self {
            id: host.id().to_string(),
            label: host.label().to_string(),
            endpoint: EndpointDto::from(host.endpoint()),
            auth: AuthDto::from(host.auth()),
            group: host.group().map(ToString::to_string),
        }
    }
}

/// What an import of `~/.ssh/config` did.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(
    rename = "TImportReport",
    export,
    export_to = "../../ui/inventory/inventory-contract.ts"
)]
pub struct ImportReportDto {
    pub imported: Vec<HostDto>,
    pub already_saved: u32,
    pub invalid: Vec<InvalidEntryDto>,
}

/// A config entry left out of an import, and why.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(
    rename = "TInvalidEntry",
    export,
    export_to = "../../ui/inventory/inventory-contract.ts"
)]
pub struct InvalidEntryDto {
    pub alias: String,
    pub reason: String,
}

impl From<&ImportReport> for ImportReportDto {
    fn from(report: &ImportReport) -> Self {
        Self {
            imported: report.imported.iter().map(HostDto::from).collect(),
            already_saved: u32::try_from(report.already_saved).unwrap_or(u32::MAX),
            invalid: report
                .invalid
                .iter()
                .map(|(alias, reason)| InvalidEntryDto {
                    alias: alias.clone(),
                    reason: reason.clone(),
                })
                .collect(),
        }
    }
}

/// A new host (no `id`) or the changes to a saved one.
#[derive(Debug, Clone, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(
    rename = "TSaveHostRequest",
    export,
    export_to = "../../ui/inventory/inventory-contract.ts"
)]
pub struct SaveHostRequest {
    #[ts(optional)]
    pub id: Option<String>,
    pub label: String,
    pub endpoint: EndpointDto,
    pub auth: AuthDto,
    /// Empty or missing means no group.
    #[ts(optional)]
    pub group: Option<String>,
}

impl SaveHostRequest {
    pub fn into_domain(self) -> Result<Host, CommandError> {
        let invalid = |error: &dyn ToString| CommandError::invalid_input(&error.to_string());
        let id = match self.id.as_deref() {
            Some(id) => id.parse::<HostId>().map_err(|error| invalid(&error))?,
            None => HostId::new(),
        };
        let group = match self.group.as_deref().map(str::trim) {
            None | Some("") => None,
            Some(name) => Some(GroupName::parse(name).map_err(|error| invalid(&error))?),
        };

        Ok(Host::new(
            id,
            HostLabel::parse(&self.label).map_err(|error| invalid(&error))?,
            self.endpoint.into_domain()?,
            self.auth.into_domain()?,
            group,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn request(id: Option<String>, group: Option<&str>) -> SaveHostRequest {
        SaveHostRequest {
            id,
            label: "web".to_owned(),
            endpoint: EndpointDto {
                address: "10.0.0.5".to_owned(),
                port: 22,
                username: "ubuntu".to_owned(),
            },
            auth: AuthDto::KeyFile {
                path: "~/.ssh/id_ed25519".to_owned(),
            },
            group: group.map(ToOwned::to_owned),
        }
    }

    #[test]
    fn a_request_without_an_id_creates_a_new_host() {
        let first = request(None, None).into_domain().unwrap();
        let second = request(None, None).into_domain().unwrap();
        assert_ne!(first.id(), second.id());
    }

    #[test]
    fn a_request_with_an_id_keeps_it() {
        let id = HostId::new();
        let host = request(Some(id.to_string()), None).into_domain().unwrap();
        assert_eq!(host.id(), id);
    }

    #[test]
    fn a_blank_group_means_no_group() {
        let host = request(None, Some("  ")).into_domain().unwrap();
        assert_eq!(host.group(), None);
    }

    #[test]
    fn a_saved_host_round_trips_through_the_dto() {
        let host = request(None, Some("Cloud")).into_domain().unwrap();
        let dto = HostDto::from(&host);
        assert_eq!(dto.group.as_deref(), Some("Cloud"));
        assert_eq!(dto.endpoint.address, "10.0.0.5");
    }
}
