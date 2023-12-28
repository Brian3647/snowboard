use crate::Request;
use crate::ResponseLike;

pub mod listener;

/// The size of the buffer used to read incoming requests.
/// It's set to 8KiB by default.
pub const DEFAULT_BUFFER_SIZE: usize = 1024 * 8;

use mio::net::TcpStream;
use std::{io, net::SocketAddr};

#[cfg(feature = "tls")]
use native_tls::{TlsAcceptor, TlsStream};

/// A TCP stream
#[cfg(not(feature = "tls"))]
pub type Stream = TcpStream;

/// A TLS stream.
#[cfg(feature = "tls")]
pub type Stream = TlsStream<TcpStream>;

#[cfg(feature = "websocket")]
use crate::ws::{maybe_websocket, WebSocket};

#[cfg(feature = "async")]
use std::future::Future;

use listener::Listener;

/// Single threaded listener made for simpler servers.
pub struct Server {
	addr: SocketAddr,
	buffer_size: usize,
	insert_default_headers: bool,
	#[cfg(feature = "tls")]
	tls_acceptor: TlsAcceptor,
	#[cfg(feature = "websocket")]
	ws_handler: Option<(&'static str, fn(WebSocket<&mut Stream>))>,
}

/// Simple rust TCP HTTP server.
impl Server {
	/// Create a new server instance.
	/// The server will listen on the given address.
	#[cfg(not(feature = "tls"))]
	pub fn new(addr: SocketAddr) -> Self {
		Self {
			addr,
			buffer_size: DEFAULT_BUFFER_SIZE,
			#[cfg(feature = "websocket")]
			ws_handler: None,
			insert_default_headers: false,
		}
	}

	/// Create a new server instance with TLS.
	/// The server will listen on the given address.
	#[cfg(feature = "tls")]
	pub fn new_with_tls(addr: SocketAddr, tls_acceptor: TlsAcceptor) -> Self {
		Self {
			addr,
			buffer_size: DEFAULT_BUFFER_SIZE,
			tls_acceptor,
			#[cfg(feature = "websocket")]
			ws_handler: None,
			insert_default_headers: false,
		}
	}

	/// Enables automatic insertion of default headers in responses.
	/// This includes `Server`, `Date` and `Content-Length`.
	pub fn with_default_headers(mut self) -> Self {
		self.insert_default_headers = true;
		self
	}

	/// Get the address the server is listening on.
	#[inline]
	pub fn addr(&self) -> SocketAddr {
		self.addr
	}

	/// Get the address the server is listening on as a string,
	/// formatted to be able to use it as a link.
	#[inline]
	pub fn pretty_addr(&self) -> String {
		crate::util::format_addr(self.addr)
	}

	/// Set the buffer size used to read incoming requests.
	/// The default buffer size is 8KiB.
	///
	/// If you want requests to actually get parsed, the buffer size must be greater than 5,
	/// the minimum size of a "valid" HTTP request (`GET /`)
	///
	/// Consider using a smaller buffer size if your server
	/// doesn't require bodies in requests, and a larger one if
	/// you expect large payloads. 8KiB is a good default, tho.
	///
	/// Note that requests bigger than the buffer size will be rejected,
	/// sending a `413 Payload Too Large` response.
	pub fn set_buffer_size(&mut self, size: usize) {
		self.buffer_size = size;
	}

	/// Sets the buffer size and returns self.
	/// See [`set_buffer_size`].
	pub fn with_buffer_size(mut self, size: usize) -> Self {
		self.buffer_size = size;
		self
	}

	/// Set a handler for WebSocket connections.
	/// The handler function will be called when a WebSocket connection is received.
	///
	/// # Example
	/// ```rust
	/// use snowboard::{response, Server};
	///
	/// Server::new("localhost:8080")
	///     .expect("Failed to start server")
	///     .on_websocket("/ws", |ws| {
	///         // Handle the WebSocket connection
	///     })
	///    .run(|_| response!(ok)); // Handle HTTP requests
	///
	#[cfg(feature = "websocket")]
	pub fn on_websocket(mut self, path: &'static str, handler: fn(WebSocket<&mut Stream>)) -> Self {
		self.ws_handler = Some((path, handler));
		self
	}

	/// Generates a new `Listener` instance from `self.addr`.
	fn listener(&self) -> io::Result<Listener> {
		Listener::new(self.addr)
	}

	/// Uses `self.acceptor.loop` to accept connections parsing the `Request`
	#[inline(always)]
	fn r#loop(self, handler: impl Fn(Stream, Request)) -> io::Result<()> {
		self.listener()?.r#loop(move |stream, ip| {
			if let Ok((stream, request)) = self.try_accept(stream, ip) {
				handler(stream, request);
			}
		})
	}

	/// Runs the server synchronously using multiple threads.
	pub fn run<T: ResponseLike>(
		self,
		handler: impl Fn(Request) -> T + Send + 'static + Clone,
	) -> io::Result<()> {
		#[cfg(feature = "websocket")]
		let ws_handler = self.ws_handler.clone();

		let should_insert = self.insert_default_headers;

		// Needed for avoiding warning when compiling without the websocket feature.
		#[cfg_attr(not(feature = "websocket"), allow(unused_mut))]
		self.r#loop(|mut stream, mut request| {
			let handler = handler.clone();

			std::thread::spawn(move || {
				#[cfg(feature = "websocket")]
				if maybe_websocket(ws_handler, &mut stream, &mut request) {
					return;
				};

				let _ = handler(request)
					.to_response()
					.maybe_add_defaults(should_insert)
					.send_to(&mut stream);
			});
		})
	}

	/// Runs the server asynchronously using multiple threads.
	#[cfg(feature = "async")]
	pub fn run_async<F, T, R>(self, handler: F) -> io::Result<()>
	where
		F: Fn(Request) -> R + Send + 'static + Clone,
		R: Future<Output = T> + Send + 'static,
		T: ResponseLike,
	{
		#[cfg(feature = "websocket")]
		let ws_handler = self.ws_handler.clone();

		let should_insert = self.insert_default_headers;

		// Needed for avoiding warning when compiling without the websocket feature.
		#[cfg_attr(not(feature = "websocket"), allow(unused_mut))]
		self.r#loop(|mut stream, mut request| {
			let handler = handler.clone();

			async_std::task::spawn(async move {
				#[cfg(feature = "websocket")]
				if maybe_websocket(ws_handler, &mut stream, &mut request) {
					return Ok(());
				};

				handler(request)
					.await
					.to_response()
					.maybe_add_defaults(should_insert)
					.send_to(&mut stream)
			});
		})
	}
}

// This is a workaround to avoid having to copy documentation.

impl Server {
	/// Try to accept a new incoming request safely.
	/// Returns an error if the request could not be read, is empty or invalid.
	/// The request will be read into a buffer and parsed into a `Request` instance.
	/// The buffer size can be changed with `Server::set_buffer_size()`.
	///
	/// # Example
	/// ```rust
	/// use snowboard::{Request, Response, Server};
	///
	/// let server = Server::new("localhost:8080").expect("failed to start server");
	///
	/// while let Ok((stream, request)) = server.try_accept() {
	///     // Handle a request
	/// }
	/// ```
	#[inline]
	pub fn try_accept(&self, stream: TcpStream, ip: SocketAddr) -> io::Result<(Stream, Request)> {
		self.try_accept_inner(stream, ip)
	}

	#[cfg(not(feature = "tls"))]
	#[inline]
	fn try_accept_inner(&self, stream: TcpStream, ip: SocketAddr) -> io::Result<(Stream, Request)> {
		self.handle_request(stream, ip)
	}

	#[cfg(feature = "tls")]
	fn try_accept_inner(
		&self,
		mut tcp_stream: TcpStream,
		ip: SocketAddr,
	) -> io::Result<(Stream, Request)> {
		// Using `tls_acceptor` directly consumes the first 4 bytes of the stream,
		// making redirects hard (and maybe impossible) to implement. `native_tls` uses
		// different implementations (even externally) for `TlsAcceptor`, so the only
		// safe way is this.

		let mut buffer = [0; 2];
		tcp_stream.peek(&mut buffer)?;

		if buffer == [0x16, 0x03] {
			// This looks like a TLS handshake.
			match self.tls_acceptor.accept(tcp_stream) {
				Ok(tls_stream) => self.handle_request(tls_stream, ip),
				Err(_) => {
					// Continue to the next connection
					Err(io::Error::from(io::ErrorKind::ConnectionAborted))
				}
			}
		} else {
			// This doesn't look like a TLS handshake. Handle it as a non-TLS request.
			self.handle_not_tls(&mut tcp_stream)?;
			Err(io::Error::from(io::ErrorKind::ConnectionAborted))
		}
	}

	fn handle_request<T: io::Write + io::Read>(
		&self,
		mut stream: T,
		ip: SocketAddr,
	) -> io::Result<(T, Request)> {
		let mut buffer: Vec<u8> = vec![0; self.buffer_size];
		let payload_size = stream.read(&mut buffer)?;

		if payload_size > self.buffer_size {
			crate::response!(payload_too_large).send_to(&mut stream)?;
			return Err(io::Error::new(
				io::ErrorKind::InvalidInput,
				"Payload too large",
			));
		}

		if payload_size == 0 {
			crate::response!(bad_request).send_to(&mut stream)?;
			return Err(io::Error::new(io::ErrorKind::InvalidInput, "Empty request"));
		}

		let req = match Request::new(&buffer[..payload_size], ip) {
			Some(req) => req,
			None => return Err(io::Error::from(io::ErrorKind::InvalidInput)),
		};

		Ok((stream, req))
	}

	// Extremely simple HTTP to HTTPS redirect.
	#[cfg(feature = "tls")]
	fn handle_not_tls<T: io::Read + io::Write>(&self, mut stream: T) -> io::Result<()> {
		let mut buffer: Vec<u8> = vec![0; self.buffer_size];

		stream.read(&mut buffer)?;

		let mut path = vec![];
		let mut in_path = false;

		for byte in buffer.iter() {
			if *byte == b' ' {
				if in_path {
					break;
				} else {
					in_path = true;
					continue;
				}
			}

			if in_path {
				path.push(*byte);
			}
		}

		let path = String::from_utf8_lossy(&path).to_string();

		crate::response!(
			moved_permanently,
			[],
			crate::headers! {
				"Location" => format!("https://{}{}", self.pretty_addr(), path),
				"Connection" => "keep-alive",
				"Content-Length" => 0
			}
		)
		.send_to(&mut stream)?;

		Ok(())
	}
}
