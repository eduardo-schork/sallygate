use sallygate_application::transfer::{
    FolderListing, LocalFilesError, TransferError, TransferProgress,
};
use sallygate_domain::transfer::{EntryKind, FileEntry};
use serde::{Deserialize, Serialize};
use tauri::ipc::Channel;
use ts_rs::TS;

use crate::command_error::{CommandError, CommandErrorKind};
use crate::sessions::from_session_command_error;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(
    rename = "TEntryKind",
    export,
    export_to = "../../ui/transfer/transfer-contract.ts"
)]
pub enum EntryKindDto {
    File,
    Folder,
    Link,
}

impl From<EntryKind> for EntryKindDto {
    fn from(kind: EntryKind) -> Self {
        match kind {
            EntryKind::File => Self::File,
            EntryKind::Folder => Self::Folder,
            EntryKind::Link => Self::Link,
        }
    }
}

impl From<EntryKindDto> for EntryKind {
    fn from(kind: EntryKindDto) -> Self {
        match kind {
            EntryKindDto::File => Self::File,
            EntryKindDto::Folder => Self::Folder,
            EntryKindDto::Link => Self::Link,
        }
    }
}

/// One file, folder or link, on this computer or on the server.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(
    rename = "TFileEntry",
    export,
    export_to = "../../ui/transfer/transfer-contract.ts"
)]
pub struct FileEntryDto {
    pub name: String,
    pub path: String,
    pub kind: EntryKindDto,
    /// A JavaScript number holds sizes up to 9 PB exactly.
    #[ts(type = "number")]
    pub size_bytes: u64,
    /// Seconds since 1970.
    #[ts(optional, type = "number")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_at: Option<u64>,
    /// Unix mode bits, like `0o644`.
    #[ts(optional)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<u32>,
}

impl From<&FileEntry> for FileEntryDto {
    fn from(entry: &FileEntry) -> Self {
        Self {
            name: entry.name.clone(),
            path: entry.path.clone(),
            kind: entry.kind.into(),
            size_bytes: entry.size_bytes,
            modified_at: entry.modified_at,
            permissions: entry.permissions,
        }
    }
}

/// A folder and what is in it, folders first.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(
    rename = "TFolderListing",
    export,
    export_to = "../../ui/transfer/transfer-contract.ts"
)]
pub struct FolderListingDto {
    pub path: String,
    pub entries: Vec<FileEntryDto>,
}

impl From<&FolderListing> for FolderListingDto {
    fn from(folder: &FolderListing) -> Self {
        Self {
            path: folder.path.clone(),
            entries: folder.entries.iter().map(FileEntryDto::from).collect(),
        }
    }
}

/// How far a transfer got.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(
    rename = "TTransferProgress",
    export,
    export_to = "../../ui/transfer/transfer-contract.ts"
)]
pub struct TransferProgressDto {
    #[ts(type = "number")]
    pub done_bytes: u64,
    #[ts(type = "number")]
    pub total_bytes: u64,
}

/// Sends a transfer's progress to the UI over the channel it started the transfer with.
pub struct ChannelTransferProgress(pub Channel<TransferProgressDto>);

impl TransferProgress for ChannelTransferProgress {
    fn advanced(&self, done_bytes: u64, total_bytes: u64) {
        // A failed send means the window that listened is gone; the copy goes on regardless.
        let _ = self.0.send(TransferProgressDto {
            done_bytes,
            total_bytes,
        });
    }
}

pub fn from_transfer_error(error: &TransferError) -> CommandError {
    match error {
        TransferError::Session(error) => from_session_command_error(error),
        TransferError::Unavailable(_) => {
            CommandError::new(CommandErrorKind::FilesUnavailable, error)
        }
        TransferError::Remote(_) => CommandError::new(CommandErrorKind::FileOperationFailed, error),
    }
}

pub fn from_local_files_error(error: &LocalFilesError) -> CommandError {
    CommandError::new(CommandErrorKind::FileOperationFailed, error)
}
