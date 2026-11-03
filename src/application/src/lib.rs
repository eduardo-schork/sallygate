//! Application layer: one use case per user intent, and the ports (traits)
//! those use cases need from the outside world.

pub mod credentials;
pub mod inventory;
pub mod sessions;
pub mod snippets;
pub mod transfer;
pub mod trust;
