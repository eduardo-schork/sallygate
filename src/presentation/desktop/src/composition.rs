//! The composition root: the only place that knows which adapter backs each port.

use std::path::Path;
use std::sync::Arc;

use sallygate_application::inventory::{
    DeleteHost, HostRepository, ImportSshConfig, ListHosts, SaveHost,
};
use sallygate_application::sessions::{
    CloseSession, OpenSession, ResizeSession, SessionRegistry, WriteToSession,
};
use sallygate_application::transfer::{BrowseRemoteFiles, OpenFileSystems, TransferFiles};
use sallygate_application::trust::{KnownHosts, TrustHostKey};
use sallygate_infrastructure::inventory::{JsonHostRepository, SshConfigFile};
use sallygate_infrastructure::sessions::RusshConnector;
use sallygate_infrastructure::trust::KnownHostsFile;
use thiserror::Error;

/// The use cases the Tauri commands call, shared as Tauri state.
pub struct AppServices {
    pub list_hosts: ListHosts,
    pub save_host: SaveHost,
    pub delete_host: DeleteHost,
    pub import_ssh_config: ImportSshConfig,
    pub open_session: OpenSession,
    pub write_to_session: WriteToSession,
    pub resize_session: ResizeSession,
    pub close_session: CloseSession,
    pub trust_host_key: TrustHostKey,
    pub browse_remote_files: BrowseRemoteFiles,
    pub transfer_files: TransferFiles,
}

#[derive(Debug, Error)]
pub enum CompositionError {
    #[error("the home folder could not be found, so ~/.ssh/known_hosts is unavailable")]
    NoHomeFolder,
}

/// Builds the use cases on top of the real adapters. App data (saved hosts) lives in
/// `app_data_folder`; trusted server keys in the user's `~/.ssh/known_hosts`; hosts to import
/// in `~/.ssh/config`.
///
/// # Errors
///
/// Returns an error when the user's home folder cannot be found.
pub fn compose(app_data_folder: &Path) -> Result<AppServices, CompositionError> {
    let home = std::env::home_dir().ok_or(CompositionError::NoHomeFolder)?;
    let ssh_folder = home.join(".ssh");
    let known_hosts: Arc<dyn KnownHosts> =
        Arc::new(KnownHostsFile::new(ssh_folder.join("known_hosts")));
    let local_username = std::env::var("USER").unwrap_or_default();
    let ssh_config = Arc::new(SshConfigFile::new(
        ssh_folder.join("config"),
        ssh_folder,
        local_username,
    ));
    let registry = Arc::new(SessionRegistry::new());
    let file_systems = Arc::new(OpenFileSystems::new(Arc::clone(&registry)));
    let hosts: Arc<dyn HostRepository> =
        Arc::new(JsonHostRepository::new(app_data_folder.join("hosts.json")));

    Ok(AppServices {
        list_hosts: ListHosts::new(Arc::clone(&hosts)),
        save_host: SaveHost::new(Arc::clone(&hosts)),
        delete_host: DeleteHost::new(Arc::clone(&hosts)),
        import_ssh_config: ImportSshConfig::new(ssh_config, hosts),
        open_session: OpenSession::new(
            Arc::new(RusshConnector::new()),
            Arc::clone(&known_hosts),
            Arc::clone(&registry),
        ),
        write_to_session: WriteToSession::new(Arc::clone(&registry)),
        resize_session: ResizeSession::new(Arc::clone(&registry)),
        close_session: CloseSession::new(registry),
        trust_host_key: TrustHostKey::new(known_hosts),
        browse_remote_files: BrowseRemoteFiles::new(Arc::clone(&file_systems)),
        transfer_files: TransferFiles::new(file_systems),
    })
}
