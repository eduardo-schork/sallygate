use sallygate_domain::snippets::{Snippet, SnippetCommand, SnippetId, SnippetName};

/// Commands most servers need, offered until the user saves their own list. Ubuntu and Debian
/// first, like the owner's servers.
const STARTERS: [(&str, &str); 8] = [
    (
        "Update the system",
        "sudo apt update && sudo apt upgrade -y",
    ),
    ("Disk space", "df -h"),
    ("Memory", "free -h"),
    ("What uses the CPU", "top -b -n 1 | head -n 20"),
    ("Running containers", "docker ps"),
    ("Follow a service's log", "journalctl -u <service> -f"),
    ("Open ports", "sudo ss -tulpn"),
    ("Biggest folders here", "du -sh * | sort -rh | head -n 10"),
];

/// The snippets a first run starts with.
#[must_use]
pub fn starter_snippets() -> Vec<Snippet> {
    STARTERS
        .iter()
        .filter_map(|(name, command)| {
            Some(Snippet {
                id: SnippetId::new(),
                name: SnippetName::parse(name).ok()?,
                command: SnippetCommand::parse(command).ok()?,
            })
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_starter_is_a_valid_snippet() {
        assert_eq!(starter_snippets().len(), STARTERS.len());
    }
}
