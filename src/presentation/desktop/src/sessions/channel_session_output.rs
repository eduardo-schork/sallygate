use base64::Engine;
use base64::engine::general_purpose::STANDARD as BASE64;
use sallygate_application::sessions::SessionOutput;
use sallygate_domain::sessions::SessionEnd;
use tauri::ipc::Channel;

use super::contract::SessionEventDto;

/// Sends one session's output to the UI over the channel it opened the session with.
pub struct ChannelSessionOutput {
    channel: Channel<SessionEventDto>,
}

impl ChannelSessionOutput {
    pub fn new(channel: Channel<SessionEventDto>) -> Self {
        Self { channel }
    }

    // A failed send means the window that listened is gone; there is no one left to tell.
    fn send(&self, event: SessionEventDto) {
        let _ = self.channel.send(event);
    }
}

impl SessionOutput for ChannelSessionOutput {
    fn data(&self, bytes: &[u8]) {
        self.send(SessionEventDto::Output {
            data: BASE64.encode(bytes),
        });
    }

    fn ended(&self, end: SessionEnd) {
        self.send(match end {
            SessionEnd::Exited(exit_status) => SessionEventDto::Exited { exit_status },
            SessionEnd::Disconnected(message) => SessionEventDto::Disconnected { message },
        });
    }
}
