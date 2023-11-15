#![forbid(missing_docs)]
#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

mod request;
mod response;
mod server;
mod url;
mod util;

pub use request::Request;
pub use response::{Headers, Response, ResponseLike, DEFAULT_HTTP_VERSION};
pub use server::{Server, DEFAULT_BUFFER_SIZE};
pub use url::Url;
pub use util::{HttpVersion, Method};

#[cfg(feature = "tls")]
// Re-export needed structs for `Server::new(...)` with TLS.
pub use native_tls::{Identity, TlsAcceptor};

/// A type alias for `std::io::Result<()>`
/// used in `Server::new()?.run(...)`.
///
/// `Server::run` returns type `!` (never) so using `Ok(())` is not needed.
pub type Result = std::io::Result<()>;
