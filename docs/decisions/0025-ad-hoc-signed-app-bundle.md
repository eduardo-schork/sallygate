# ADR-0025: The app bundle is ad-hoc signed as `dev.sallygate.app`

- **Status:** Accepted
- **Date:** 2026-11-08

## Context

Connecting to a server on the local network (`192.168.x.x`) failed with "No route to host" while the `ssh` command reached it. macOS grants local network access per app, keyed by its code signature. Without an Apple certificate, the bundle was only signed by the linker with a random identifier that changes every build, and its `Info.plist` (with `NSLocalNetworkUsageDescription`) was not part of the signature: macOS refused without asking.

## Decision

`bundle.macOS.signingIdentity` is `"-"`: Tauri ad-hoc signs the whole app, sealing `Info.plist` and naming it `dev.sallygate.app`. macOS then asks once for local network access and remembers the answer for that identity.

## Consequences

- Ad-hoc signing is not Apple's trust: Gatekeeper still asks before the first open of a downloaded app (ADR-0021).
- The hardened runtime comes with this signature; nothing in the app needs an exception so far.
- Every build, local or in CI, uses the same identity, so a permission granted once keeps working across updates.
