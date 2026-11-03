//! Infrastructure layer: adapters that implement the application ports with
//! concrete technology (SSH, file system, OS keychain).

pub mod inventory;
mod json_file;
pub mod sessions;
pub mod snippets;
pub mod transfer;
pub mod trust;
