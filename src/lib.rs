#![forbid(unsafe_code, clippy::panic)]
#![deny(missing_docs, clippy::missing_docs_in_private_items, clippy::perf)]
#![warn(clippy::cognitive_complexity, rust_2018_idioms)]
#![doc = include_str!("../README.md")]

mod macros;
mod request;
mod response;
mod server;
mod url;
mod util;

#[cfg(feature = "websocket")]
mod ws;

pub use request::Request;
pub use response::{Headers, Response, ResponseLike, DEFAULT_HTTP_VERSION};
pub use server::listener::Listener;
pub use server::shutdown::Shutdown;
pub use server::{Server, Stream, DEFAULT_BUFFER_SIZE};
pub use url::Url;
pub use util::{HttpVersion, Method};

#[cfg(feature = "websocket")]
/// A WebSocket connection.
pub type WebSocket<'a> = tungstenite::WebSocket<&'a mut Stream>;

#[cfg(feature = "tls")]
// Re-export needed structs for `Server::new(...)` with TLS.
pub use async_native_tls::{Identity, TlsAcceptor};

/// A type alias for `std::io::Result<()>`
/// used in `Server::new()?.run(...)`.
///
/// `Server::run` returns type `!` (never) so using `Ok(())` is not needed.
pub type Result = std::io::Result<()>;
