use crate::request::Request;
use crate::response::Response;

/// The size of the buffer used to read incoming requests.
/// It's set to 8KiB by default.
pub const DEFAULT_BUFFER_SIZE: usize = 1024 * 8;

use std::{
	io,
	net::{SocketAddr, TcpListener, TcpStream, ToSocketAddrs},
};

#[cfg(feature = "tls")]
use native_tls::{TlsAcceptor, TlsStream};

#[cfg(feature = "async")]
use std::future::Future;

/// Single threaded listener made for simpler servers.
pub struct Server {
	acceptor: TcpListener,
	buffer_size: usize,
	#[cfg(feature = "tls")]
	tls_acceptor: TlsAcceptor,
}

/// Simple rust TCP HTTP server.
impl Server {
	/// Create a new server instance.
	/// The server will listen on the given address.
	/// The address must be in the format of `ip:port`.
	#[cfg(not(feature = "tls"))]
	pub fn new(addr: impl ToSocketAddrs) -> io::Result<Self> {
		Ok(Self {
			acceptor: TcpListener::bind(addr)?,
			buffer_size: DEFAULT_BUFFER_SIZE,
		})
	}

	/// Create a new server instance with TLS support, based on
	/// a given TLS acceptor and adress.
	///
	/// The server will listen on the given address.
	/// The address must be in the format of `ip:port`.
	#[cfg(feature = "tls")]
	pub fn new(addr: impl ToSocketAddrs, acceptor: TlsAcceptor) -> io::Result<Self> {
		Ok(Self {
			acceptor: TcpListener::bind(addr)?,
			tls_acceptor: acceptor,
			buffer_size: DEFAULT_BUFFER_SIZE,
		})
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

	/// Run a multi-thread listener from a handler function.
	/// The handler function will be called when a request is received.
	/// The handler function must return a `Response` instance. Meant for servers
	/// where passing data to the handler is not needed.
	///
	/// # Example
	/// ```rust
	/// use snowboard::{response, Server};
	///
	/// Server::new("localhost:8080").expect("Failed to start server").run(|_| response!(ok));
	/// ```
	#[cfg(not(feature = "async"))]
	pub fn run(self, handler: impl Fn(Request) -> Response + Send + 'static + Clone) -> ! {
		for (mut stream, request) in self {
			let handler = handler.clone();

			std::thread::spawn(move || {
				if let Err(e) = handler(request).send_to(&mut stream) {
					eprintln!("Error writing response: {:?}", e);
				};
			});
		}

		unreachable!("Server::run() should never return")
	}

	/// Runs the server asynchronously.
	///
	/// This function takes a handler function as an argument. The handler function is expected to be a
	/// function that takes a `Request` and returns a `Future` that resolves to a `Response`.
	///
	/// The handler function is cloned for each request, and each request is processed in a separate
	/// async task. This means that multiple requests can be processed concurrently.
	///
	/// This function is only available when the `async` feature is enabled.
	///
	/// # Example
	/// ```rust
	/// use snowboard::{response, Server};
	///
	/// Server::new("localhost:8080").expected("Failed to start server").run(async |_| response!(ok));
	/// ```
	#[cfg(feature = "async")]
	pub fn run<H, F>(self, handler: H) -> !
	where
		H: Fn(Request) -> F + Clone + Send + 'static + Sync,
		F: Future<Output = Response> + Send + 'static,
	{
		for (mut stream, request) in self {
			let handler = handler.clone();

			async_std::task::spawn(async move {
				if let Err(e) = handler(request).await.send_to(&mut stream) {
					eprintln!("Error writing response: {:?}", e);
				};
			});
		}

		unreachable!("Server::run() should never return")
	}
}

// This is a workaround to avoid having to copy documentation.

#[cfg(not(feature = "tls"))]
type Stream = TcpStream;

#[cfg(feature = "tls")]
type Stream = TlsStream<TcpStream>;

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
	/// loop {
	///    match server.try_accept() {
	///       Ok((stream, request)) => {
	///         if let Ok(request) = request {
	///            // Handle request
	///        }
	///      },
	///      Err(_) => {
	///        // Handle error
	///      }
	///   }
	/// }
	/// ```
	pub fn try_accept(&self) -> io::Result<(Stream, Result<Request, String>)> {
		self.try_accept_inner()
	}

	#[cfg(not(feature = "tls"))]
	fn try_accept_inner(&self) -> io::Result<(TcpStream, Result<Request, String>)> {
		let (stream, ip) = self.acceptor.accept()?;

		self.handle_request(stream, ip)
	}

	#[cfg(feature = "tls")]
	fn try_accept_inner(&self) -> io::Result<(TlsStream<TcpStream>, Result<Request, String>)> {
		let (tcp_stream, ip) = self.acceptor.accept()?;
		let tls_stream = self.tls_acceptor.accept(tcp_stream).map_err(|tls_error| {
			io::Error::new(io::ErrorKind::InvalidInput, tls_error.to_string())
		})?;

		self.handle_request(tls_stream, ip)
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
			// If the request is invalid, we just ignore it and try again.
			Ok((_, Err(_))) => self.next(),
			Err(e) => {
				eprintln!("Server generated error: {:#?}", e);
				None
			}
		}
	}
}
