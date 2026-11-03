# ADR-0018: Port forwarding rules, one SSH connection each

- **Status:** Accepted
- **Date:** 2026-11-03

## Context

Stage 4 adds port forwarding, as in the owner's prototype: saved rules with an on/off switch, such as a database on a server reached from `localhost`, or a SOCKS proxy through a server. A rule must work without a terminal tab open.

## Decision

A rule (`tunnels` context) names a saved host and a kind:

- **Local** (`ssh -L`): `127.0.0.1:<local port>` reaches `<address>:<port>` as the server sees it.
- **Dynamic** (`ssh -D`): a SOCKS5 proxy on `127.0.0.1:<local port>` (no login, CONNECT only).

Rules live in `tunnels.json`. Turning one on opens its own SSH connection with the same log-in step terminal sessions use (server key check, then key file or password), binds the local port first so a busy port fails before any password prompt, and forwards each accepted connection over a `direct-tcpip` channel. Turning it off stops listening and closes the connection. A password or passphrase follows ADR-0015: typed when the rule is turned on, used once.

## Alternatives considered

- Reuse a terminal session's connection: rules would stop when the tab closes, and a rule could not start without a tab.
- Listen on all interfaces: anyone on the network could use the server through the Mac.
- Remote forwarding (`ssh -R`) now: rarely needed by the owner; it comes when asked.

## Consequences

- Each rule that is on holds one SSH connection; rules are not shared between rules or tabs.
- Rules do not turn on by themselves when the app starts.
- A dropped connection is not detected yet (a `shortcut:` in `russh_tunnel_opener.rs`).
- The Docker test server allows TCP forwarding so integration tests cover both kinds.
