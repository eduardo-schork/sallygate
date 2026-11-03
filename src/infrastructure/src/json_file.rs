//! Small JSON files in the app data folder, read whole and written atomically.

use std::fs;
use std::io::ErrorKind;
use std::path::Path;

use serde::Serialize;
use serde::de::DeserializeOwned;

/// The file's contents, or `None` when it does not exist yet.
pub(crate) fn read<T: DeserializeOwned>(path: &Path) -> Result<Option<T>, String> {
    let contents = match fs::read_to_string(path) {
        Ok(contents) => contents,
        Err(error) if error.kind() == ErrorKind::NotFound => return Ok(None),
        Err(error) => return Err(error.to_string()),
    };
    serde_json::from_str(&contents)
        .map(Some)
        .map_err(|error| error.to_string())
}

/// Writes to a temporary file first, so a crash mid-write never leaves a broken file.
pub(crate) fn write_atomically<T: Serialize>(path: &Path, value: &T) -> Result<(), String> {
    let contents = serde_json::to_string_pretty(value).map_err(|error| error.to_string())?;
    if let Some(folder) = path.parent() {
        fs::create_dir_all(folder).map_err(|error| error.to_string())?;
    }
    let temporary_path = path.with_extension("json.tmp");
    fs::write(&temporary_path, contents).map_err(|error| error.to_string())?;
    fs::rename(&temporary_path, path).map_err(|error| error.to_string())
}
