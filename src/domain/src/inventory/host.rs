use crate::credentials::AuthMethod;

use super::{GroupName, HostId, HostLabel, SshEndpoint};

/// A server the user saved to connect to again.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Host {
    id: HostId,
    label: HostLabel,
    endpoint: SshEndpoint,
    auth: AuthMethod,
    group: Option<GroupName>,
}

impl Host {
    #[must_use]
    pub fn new(
        id: HostId,
        label: HostLabel,
        endpoint: SshEndpoint,
        auth: AuthMethod,
        group: Option<GroupName>,
    ) -> Self {
        Self {
            id,
            label,
            endpoint,
            auth,
            group,
        }
    }

    #[must_use]
    pub fn id(&self) -> HostId {
        self.id
    }

    #[must_use]
    pub fn label(&self) -> &HostLabel {
        &self.label
    }

    #[must_use]
    pub fn endpoint(&self) -> &SshEndpoint {
        &self.endpoint
    }

    #[must_use]
    pub fn auth(&self) -> &AuthMethod {
        &self.auth
    }

    #[must_use]
    pub fn group(&self) -> Option<&GroupName> {
        self.group.as_ref()
    }
}
