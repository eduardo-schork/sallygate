# Glossary

## Domain

- **SSH**: protocol for running a terminal on a remote server over an encrypted connection. Default port 22.
- **SFTP**: file transfer that runs inside an SSH connection: same port, same login, same key. Not related to FTP.
- **FTP**: an older, unencrypted file transfer protocol with its own server. Out of scope.
- **PTY**: pseudo-terminal. The remote side of an interactive shell; it needs to know the terminal size (cols × rows).
- **Port forwarding / tunnel**: carrying a TCP connection through SSH. *Local* (`-L`) exposes a remote service on a local port; *remote* (`-R`) does the opposite; *dynamic* (`-D`) is a SOCKS proxy.
- **Jump host**: a server you connect to first in order to reach another one behind it (`ProxyJump`).
- **Known hosts**: the list of server key fingerprints already trusted, in `~/.ssh/known_hosts`. A changed fingerprint can mean a man-in-the-middle attack.
- **ssh-agent**: a local process that holds unlocked keys so apps can sign without reading the key file.

## Project

- **Host**: a saved server entry (label, address, port, username, auth method, group, tags).
- **Group**: a named folder of hosts.
- **Session**: one live SSH connection shown in a tab.
- **Core**: the Rust side of the app. Owns connections and secrets.
- **Vault**: the planned encrypted store for secrets, behind a master password (1.0.0).
- **Sallygate**: the gate of a sally port, the fortified passage a garrison used to leave and re-enter a fort.
