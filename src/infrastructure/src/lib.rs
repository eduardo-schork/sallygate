//! Infrastructure layer: adapters that implement the application ports with
//! concrete technology (SSH, file system, OS keychain).

pub mod sessions;
pub mod trust;
