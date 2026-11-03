/// What a remote entry is. Links are shown as they are and not followed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EntryKind {
    File,
    Folder,
    Link,
}

/// One file, folder or link inside a remote folder.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RemoteEntry {
    pub name: String,
    /// Full path on the server.
    pub path: String,
    pub kind: EntryKind,
    pub size_bytes: u64,
    /// Seconds since 1970, when the server says.
    pub modified_at: Option<u64>,
}

/// `folder` + `/` + `name`, without doubling the slash at the root.
#[must_use]
pub fn join_remote_path(folder: &str, name: &str) -> String {
    format!("{}/{name}", folder.trim_end_matches('/'))
}

/// The last part of a local or remote path, e.g. `notes.txt` for `/home/me/notes.txt`.
#[must_use]
pub fn file_name(path: &str) -> &str {
    path.trim_end_matches(['/', '\\'])
        .rsplit(['/', '\\'])
        .next()
        .unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn joins_without_doubling_the_slash() {
        assert_eq!(join_remote_path("/", "etc"), "/etc");
        assert_eq!(join_remote_path("/home/me", "a.txt"), "/home/me/a.txt");
        assert_eq!(join_remote_path("/home/me/", "a.txt"), "/home/me/a.txt");
    }

    #[test]
    fn takes_the_last_part_of_a_path() {
        assert_eq!(file_name("/Users/me/notes.txt"), "notes.txt");
        assert_eq!(file_name("/srv/app/"), "app");
        assert_eq!(file_name("notes.txt"), "notes.txt");
    }
}
