#![forbid(missing_docs)]
#![forbid(unsafe_code)]

//! Snowboard: A simple HTTP server library in Rust.
//!
//! Support for sync & async functions, depending on feature flags.
//! Refer to README.md for more details.

mod request;
mod response;
mod server;
mod url;
mod util;

pub use request::Request;
pub use response::Response;
pub use server::Server;
pub use url::Url;
pub use util::{HttpVersion, Method};

pub use std::net::TcpStream;

#[cfg(feature = "async")]
pub use async_std;

/// A type alias for `std::io::Result<()>`
/// used in `Server::new()?.run(...)`.
///
/// `Server::run` returns type `!` (never) so using `Ok(())` is not needed.
pub type Result = std::io::Result<()>;
