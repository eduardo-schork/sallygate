use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

use async_trait::async_trait;
use russh::{ChannelMsg, ChannelReadHalf, ChannelWriteHalf, Disconnect, client};
use russh_sftp::client::SftpSession;
use sallygate_application::sessions::{LiveSession, LiveSessionError, SessionOutput};
use sallygate_application::transfer::RemoteFiles;
use sallygate_domain::sessions::{SessionEnd, TerminalSize};

use super::server_key_check::ServerKeyCheck;
use crate::transfer::SftpRemoteFiles;

/// One shell over one SSH connection. Output is read by a background task.
pub(super) struct RusshLiveSession {
    connection: client::Handle<ServerKeyCheck>,
    /// Kept open while the session runs through it; closed with the session.
    jump: Option<client::Handle<ServerKeyCheck>>,
    writer: ChannelWriteHalf<client::Msg>,
    is_closed_by_user: Arc<AtomicBool>,
}

impl RusshLiveSession {
    pub(super) fn start(
        connection: client::Handle<ServerKeyCheck>,
        jump: Option<client::Handle<ServerKeyCheck>>,
        reader: ChannelReadHalf,
        writer: ChannelWriteHalf<client::Msg>,
        output: Arc<dyn SessionOutput>,
    ) -> Self {
        let is_closed_by_user = Arc::new(AtomicBool::new(false));
        tokio::spawn(forward_output(
            reader,
            output,
            Arc::clone(&is_closed_by_user),
        ));

        Self {
            connection,
            jump,
            writer,
            is_closed_by_user,
        }
    }
}

/// Streams the shell's output until it ends, then reports how it ended, unless
/// the user closed it (the user already knows).
async fn forward_output(
    mut reader: ChannelReadHalf,
    output: Arc<dyn SessionOutput>,
    is_closed_by_user: Arc<AtomicBool>,
) {
    let mut exit_status = None;

    while let Some(message) = reader.wait().await {
        match message {
            ChannelMsg::Data { data } | ChannelMsg::ExtendedData { data, .. } => {
                output.data(&data);
            }
            ChannelMsg::ExitStatus {
                exit_status: status,
            } => exit_status = Some(status),
            ChannelMsg::Close => break,
            _ => {}
        }
    }

    if is_closed_by_user.load(Ordering::SeqCst) {
        return;
    }
    output.ended(exit_status.map_or_else(
        || SessionEnd::Disconnected("the connection closed".to_owned()),
        SessionEnd::Exited,
    ));
}

fn session_error(error: &russh::Error) -> LiveSessionError {
    LiveSessionError(error.to_string())
}

#[async_trait]
impl LiveSession for RusshLiveSession {
    async fn write(&self, bytes: &[u8]) -> Result<(), LiveSessionError> {
        self.writer
            .data_bytes(bytes.to_vec())
            .await
            .map_err(|error| session_error(&error))
    }

    async fn resize(&self, size: TerminalSize) -> Result<(), LiveSessionError> {
        self.writer
            .window_change(u32::from(size.columns()), u32::from(size.rows()), 0, 0)
            .await
            .map_err(|error| session_error(&error))
    }

    async fn open_files(&self) -> Result<Arc<dyn RemoteFiles>, LiveSessionError> {
        let channel = self
            .connection
            .channel_open_session()
            .await
            .map_err(|error| session_error(&error))?;
        channel
            .request_subsystem(true, "sftp")
            .await
            .map_err(|error| session_error(&error))?;
        let sftp = SftpSession::new(channel.into_stream())
            .await
            .map_err(|error| LiveSessionError(error.to_string()))?;
        Ok(Arc::new(SftpRemoteFiles::new(sftp)))
    }

    async fn close(&self) -> Result<(), LiveSessionError> {
        self.is_closed_by_user.store(true, Ordering::SeqCst);
        let closed = self
            .connection
            .disconnect(Disconnect::ByApplication, "", "en")
            .await
            .map_err(|error| session_error(&error));
        if let Some(jump) = &self.jump {
            // The session is already closed; a jump host that is gone needs no goodbye.
            let _ = jump.disconnect(Disconnect::ByApplication, "", "en").await;
        }
        closed
    }
}
