use crate::Request;
use crate::ResponseLike;

/// The size of the buffer used to read incoming requests.
/// It's set to 8KiB by default.
pub const DEFAULT_BUFFER_SIZE: usize = 1024 * 8;

use std::{
	io,
	net::{SocketAddr, TcpListener, TcpStream, ToSocketAddrs},
};

#[cfg(feature = "tls")]
use native_tls::{TlsAcceptor, TlsStream};

#[cfg(not(feature = "tls"))]
type Stream = TcpStream;

#[cfg(feature = "tls")]
type Stream = TlsStream<TcpStream>;

#[cfg(feature = "websocket")]
use crate::ws::maybe_websocket;
#[cfg(feature = "websocket")]
use tungstenite::WebSocket;

#[cfg(feature = "async")]
use std::future::Future;

/// Single threaded listener made for simpler servers.
pub struct Server {
	acceptor: TcpListener,
	buffer_size: usize,
	#[cfg(feature = "tls")]
	tls_acceptor: TlsAcceptor,
	#[cfg(feature = "websocket")]
	ws_handler: Option<(&'static str, fn(WebSocket<&mut Stream>))>,
}

/// Simple rust TCP HTTP server.
impl Server {
	/// Create a new server instance.
	/// The server will listen on the given address.
	/// The address must be in the format of `ip:port`.
	pub fn new(
		addr: impl ToSocketAddrs,
		#[cfg(feature = "tls")] acceptor: TlsAcceptor,
	) -> io::Result<Self> {
		Ok(Self {
			acceptor: TcpListener::bind(addr)?,
			#[cfg(feature = "tls")]
			tls_acceptor: acceptor,
			buffer_size: DEFAULT_BUFFER_SIZE,
			#[cfg(feature = "websocket")]
			ws_handler: None,
		})
	}

	/// Get the address the server is listening on.
	#[inline]
	pub fn addr(&self) -> io::Result<SocketAddr> {
		self.acceptor.local_addr()
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

	/// Runs the server synchronously using multiple threads.
	#[cfg(not(feature = "async"))]
	pub fn run<T: ResponseLike>(
		self,
		handler: impl Fn(Request) -> T + Send + 'static + Clone,
	) -> ! {
		#[cfg(feature = "websocket")]
		let ws_handler = self.ws_handler.clone();

		// Needed for avoiding warning when compiling without the websocket feature.
		#[allow(unused_mut)]
		for (mut stream, mut request) in self {
			let handler = handler.clone();

			std::thread::spawn(move || {
				#[cfg(feature = "websocket")]
				if maybe_websocket(ws_handler, &mut stream, &mut request) {
					return Ok(());
				};

				handler(request).to_response().send_to(&mut stream)
			});
		}

		unreachable!("Server::run() should never return")
	}

	/// Runs the server asynchronously using multiple threads.
	#[cfg(feature = "async")]
	pub fn run<F, T>(self, handler: fn(Request) -> F) -> !
	where
		F: Future<Output = T> + Send + 'static,
		T: ResponseLike,
	{
		#[cfg(feature = "websocket")]
		let ws_handler = self.ws_handler.clone();

		for (mut stream, mut request) in self {
			async_std::task::spawn(async move {
				#[cfg(feature = "websocket")]
				if maybe_websocket(ws_handler, &mut stream, &mut request) {
					return Ok(());
				};

				handler(request).await.to_response().send_to(&mut stream)
			});
		}

		unreachable!("Server::run() should never return")
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
	///     if let Ok(request) = request {
	///         // Handle the request
	///     } else {
	///         // Handle an invalid request
	///     }
	/// }
	/// ```
	#[inline]
	pub fn try_accept(&self) -> io::Result<(Stream, Result<Request, String>)> {
		self.try_accept_inner()
	}

	#[cfg(not(feature = "tls"))]
	#[inline]
	fn try_accept_inner(&self) -> io::Result<(Stream, Result<Request, String>)> {
		let (stream, ip) = self.acceptor.accept()?;
		self.handle_request(stream, ip)
	}

	#[cfg(feature = "tls")]
	fn try_accept_inner(&self) -> io::Result<(Stream, Result<Request, String>)> {
		let (mut tcp_stream, ip) = self.acceptor.accept()?;
		match self.tls_acceptor.accept(tcp_stream.try_clone()?) {
			Ok(tls_stream) => self.handle_request(tls_stream, ip),
			Err(_) => {
				// Write a 426 Upgrade Required response to the stream
				crate::response!(
					upgrade_required,
					"HTTP is not supported. Use HTTPS instead."
				)
				.send_to(&mut tcp_stream)?;

				// Continue to the next connection
				Err(io::Error::from(io::ErrorKind::ConnectionAborted))
			}
		}
	}

	fn handle_request<T: io::Write + io::Read>(
		&self,
		mut stream: T,
		ip: SocketAddr,
	) -> io::Result<(T, Result<Request, String>)> {
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

		let text = String::from_utf8_lossy(&buffer).replace('\0', "");
		let req = match Request::new(&text, ip) {
			Some(req) => Ok(req),
			None => Err(text),
		};

		Ok((stream, req))
	}
}

impl Iterator for Server {
	type Item = (Stream, Request);

	fn next(&mut self) -> Option<Self::Item> {
		match self.try_accept() {
			Ok((stream, Ok(req))) => Some((stream, req)),
			// Parsing the request failed (probably due to it being empty), so we ignore it.
			Ok((_, Err(_))) => self.next(),
			// Probably unsupported error caused by TLS handshake failure, ignoring it.
			Err(e)
				if e.kind() == io::ErrorKind::ConnectionAborted
					|| e.kind() == io::ErrorKind::ConnectionReset =>
			{
				self.next()
			}
			Err(e) => {
				eprintln!("Server generated error: {:#?}", e);
				None
			}
		}
	}
}
