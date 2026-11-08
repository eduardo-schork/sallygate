/// `name`, or the first of `name (1)`, `name (2)`… that is not taken, keeping a file's
/// extension at the end (`notes (1).txt`). Copies never overwrite what is already there.
#[must_use]
pub fn free_name(name: &str, is_taken: impl Fn(&str) -> bool) -> String {
    if !is_taken(name) {
        return name.to_owned();
    }
    let (stem, extension) = match name.rsplit_once('.') {
        Some((stem, extension)) if !stem.is_empty() => (stem, format!(".{extension}")),
        _ => (name, String::new()),
    };
    (1..u32::MAX)
        .map(|number| format!("{stem} ({number}){extension}"))
        .find(|candidate| !is_taken(candidate))
        .unwrap_or_else(|| name.to_owned())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn numbers_a_taken_name_before_its_extension() {
        let taken = ["notes.txt", "notes (1).txt", "apps", ".bashrc"];
        let is_taken = |name: &str| taken.contains(&name);

        assert_eq!(free_name("report.pdf", is_taken), "report.pdf");
        assert_eq!(free_name("notes.txt", is_taken), "notes (2).txt");
        assert_eq!(free_name("apps", is_taken), "apps (1)");
        assert_eq!(free_name(".bashrc", is_taken), ".bashrc (1)");
    }
}
