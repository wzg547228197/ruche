#![deny(missing_docs)]
//! Ruche is a simple key/value storage code in rust.

#[macro_use] extern crate log;

mod client;
mod error;
mod store;
mod request;
mod response;
mod server;

pub use error::{RucheError, RucheResult};
pub use store::RucheStore;
pub use server::RucheServer;
pub use client::RucheClient;
