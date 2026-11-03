//! Domain layer: entities, value objects and rules for each bounded context.
//! Pure code only: no I/O, no async runtime, no serialization.

pub mod credentials;
pub mod inventory;
pub mod sessions;
pub mod snippets;
pub mod transfer;
pub mod trust;
pub mod tunnels;
