#![forbid(missing_docs)]
#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

mod macros;
mod request;
mod response;
mod server;
mod url;
mod util;

#[cfg(feature = "websocket")]
pub mod ws;

pub use request::Request;
pub use response::{Headers, Response, ResponseLike, DEFAULT_HTTP_VERSION};
pub use server::{Server, DEFAULT_BUFFER_SIZE};
pub use url::Url;
pub use util::{HttpVersion, Method};

#[cfg(feature = "websocket")]
// Needed re-export
pub use tungstenite::WebSocket;

#[cfg(feature = "tls")]
// Re-export needed structs for `Server::new(...)` with TLS.
// TlsStream might also be needed for websockets.
pub use native_tls::{Identity, TlsAcceptor, TlsStream};

/// A type alias for `std::io::Result<()>`
/// used in `Server::new()?.run(...)`.
///
/// `Server::run` returns type `!` (never) so using `Ok(())` is not needed.
pub type Result = std::io::Result<()>;
