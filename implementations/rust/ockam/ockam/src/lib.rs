// This is close to no_std when serde is off, other than println macro. Would like to replace those
// with a small logging API.

#[macro_use]
extern crate alloc;

cfg_if::cfg_if! {
    if #[cfg(feature = "json")] {
    #[macro_use]
    extern crate jsonrpc_client_core;
    extern crate jsonrpc_client_http;
    extern crate serde;
    extern crate serde_json;
    pub mod transport_jsonrpc;
    }
}

// re-export the #[node] attribute macro.
pub use ockam_node_attribute::*;

#[derive(Debug)]
pub enum Error {
    WorkerRuntime,
}

pub type Result<T> = core::result::Result<T, Error>;

pub mod address;
pub mod message;
pub mod node;
pub mod queue;
pub mod registry;
pub mod route;
pub mod router;
pub mod task;
pub mod transport;
pub mod worker;
