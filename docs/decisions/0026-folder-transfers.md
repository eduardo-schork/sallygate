# ADR-0026: Folders copy whole, under a free name

- **Status:** Accepted
- **Date:** 2026-11-08
- **Supersedes in part:** [ADR-0020](0020-two-pane-file-browser.md) (folders were one file at a time)

## Context

The owner could not copy a folder from a server to the Mac: transfers handled single files only.

## Decision

- `TransferFiles` copies a file or a whole folder in both directions. For a folder it first walks the source (the server's through `RemoteFiles::list`, this Mac's through `LocalFiles::list`), then makes every destination folder, parents first, then copies each file, reporting one progress for the whole folder.
- A copied folder lands under a free name (`apps (1)`) when the name is taken, like downloaded files; nothing is overwritten. An uploaded single file still replaces a file of the same name, as before.
- Links inside a folder are skipped and never followed, so a link loop cannot make a copy endless.
- `LocalFiles` gains `kind` (to tell a dropped folder from a file) and `make_folder`; the free-name rule lives in the domain (`free_name`).

## Consequences

- A large folder is listed in full before the first byte moves; very large trees take a moment to start.
- A failure stops the copy where it is; the files already copied stay.
