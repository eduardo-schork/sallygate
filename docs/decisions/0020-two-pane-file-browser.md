# ADR-0020: The file browser shows this Mac beside the server

- **Status:** Accepted
- **Date:** 2026-11-03
- **Supersedes in part:** [ADR-0017](0017-sftp-over-the-session-connection.md) (where downloads land, and the deferred local pane)

## Context

ADR-0017 shipped a server-only file view: downloads went to the Downloads folder and uploads came from the system dialog or Finder. The owner's prototype shows two panes, "This Mac" and the server, side by side, with name, size and permissions, and a transfer bar at the bottom.

## Decision

- The Files view of a session tab has two panes: this Mac on the left, the server on the right.
- The core lists local folders through a `LocalFiles` port (`LocalFileSystem` adapter, read only) and a `BrowseLocalFiles` use case; both sides return the same `FolderListing` of `FileEntry` values, which now carry Unix permissions.
- Files go across by dragging a row onto the other pane, or with the arrow button on the row. A download lands in the folder open on the Mac side; an upload lands in the folder open on the server side. Finder drops and the Upload button still upload to the server folder.
- Only the server pane changes files (new folder, rename, delete). The Mac pane is for picking and receiving.

## Consequences

- Downloads no longer go to the Downloads folder by default; the Mac pane starts in the home folder.
- Folder uploads and downloads are still one file at a time.
