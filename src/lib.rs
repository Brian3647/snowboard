#![forbid(missing_docs)]
#![forbid(unsafe_code)]

//! # Snowboard
//!
//! An extremelly simple library for fast & simple TCP servers in rust.
//! Refer to the [README.md](https://github.com/Brian3647/snowboard/blob/README.md) for more information.

mod request;
mod response;
mod server;
mod url;
mod util;

pub use request::Request;
pub use response::{Response, DEFAULT_HTTP_VERSION};
pub use server::{Server, DEFAULT_BUFFER_SIZE};
pub use url::Url;
pub use util::{HttpVersion, Method};

pub use std::net::TcpStream;

#[cfg(feature = "async")]
pub use async_std;

#[cfg(feature = "tls")]
pub use native_tls;

/// A type alias for `std::io::Result<()>`
/// used in `Server::new()?.run(...)`.
///
/// `Server::run` returns type `!` (never) so using `Ok(())` is not needed.
pub type Result = std::io::Result<()>;
