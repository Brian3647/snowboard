use crate::request::Request;
use crate::response::Response;

#[cfg(feature = "async")]
use std::future::Future;

/// The size of the buffer used to read incoming requests.
/// It's set to 8KiB by default.
pub const DEFAULT_BUFFER_SIZE: usize = 1024 * 8;

use std::{
	io,
	net::{SocketAddr, TcpListener, TcpStream},
};

#[cfg(feature = "tls")]
use native_tls::{TlsAcceptor, TlsStream};

/// Single threaded listener made for simpler servers.
pub struct Server {
	acceptor: TcpListener,
	#[cfg(feature = "tls")]
	tls_acceptor: TlsAcceptor,
	buffer_size: usize,
}

/// Simple rust TCP HTTP server.
impl Server {
	/// Create a new server instance.
	/// The server will listen on the given address.
	/// The address must be in the format of `ip:port`.
	#[cfg(not(feature = "tls"))]
	pub fn new(addr: impl Into<String>) -> io::Result<Self> {
		let addr = addr.into();

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
	pub fn new(addr: impl Into<String>, acceptor: TlsAcceptor) -> io::Result<Self> {
		let addr = addr.into();
		let tcp = TcpListener::bind(addr)?;

		Ok(Self {
			acceptor: tcp,
			tls_acceptor: acceptor,
			buffer_size: DEFAULT_BUFFER_SIZE,
		})
	}

	/// Set the buffer size used to read incoming requests.
	/// The default buffer size is 8KiB.
	/// The buffer size must be greater than 0.
	pub fn set_buffer_size(&mut self, size: usize) {
		assert!(size > 0, "Buffer size must be greater than 0");

		self.buffer_size = size;
	}

	/// Try to accept a new incoming request safely.
	/// Returns an error if the request could not be read, is empty or invalid.
	/// The request will be read into a buffer and parsed into a `Request` instance.
	/// The buffer size can be changed with `Server::set_buffer_size()`.
	///
	/// # Example
	/// ```no_run,rust
	/// use snowboard::{Request, Response, Server};
	///
	/// let server = Server::new("localhost:8080");
	///
	/// loop {
	///    match server.try_accept() {
	///       Ok((stream, request)) => {
	///         // Handle request
	///      },
	///      Err(_) => {
	///        // Handle error
	///      }
	///   }
	/// }
	/// ```
	#[cfg(not(feature = "tls"))]
	pub fn try_accept(&self) -> io::Result<(TcpStream, Request)> {
		let (stream, ip) = self.acceptor.accept()?;

		self.handle_request(stream, ip)
	}

	/// Try to accept a new incoming request safely, using TLS.
	/// Returns an error if the request could not be read, is empty or invalid.
	/// The request will be read into a buffer and parsed into a `Request` instance.
	/// The buffer size can be changed with `Server::set_buffer_size()`.
	#[cfg(feature = "tls")]
	pub fn try_accept(&self) -> io::Result<(TlsStream<TcpStream>, Request)> {
		let (tcp_stream, ip) = self.acceptor.accept()?;
		let tls_stream = self.tls_acceptor.accept(tcp_stream).map_err(|tls_error| {
			io::Error::new(
				io::ErrorKind::InvalidInput,
				format!("TLS error: {:?}", tls_error),
			)
		})?;

		self.handle_request(tls_stream, ip)
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

		let text = String::from_utf8_lossy(&buffer).replace('\0', "");

		let req = match Request::new(text, ip) {
			Some(req) => req,
			None => {
				crate::response!(bad_request).send_to(&mut stream)?;
				return Err(io::Error::new(
					io::ErrorKind::InvalidInput,
					"Invalid request",
				));
			}
		};

		Ok((stream, req))
	}

	/// Run a multi-thread listener from a handler function.
	/// The handler function will be called when a request is received.
	/// The handler function must return a `Response` instance. Meant for servers
	/// where passing data to the handler is not needed.
	///
	/// # Example
	/// ```no_run
	/// use snowboard::{response, Server};
	///
	/// Server::new("localhost:8080").run(|_| response!(ok));
	/// ```
	#[cfg(not(feature = "async"))]
	pub fn run(self, handler: impl Fn(Request) -> Response + Send + 'static + Clone) -> ! {
		println!(
			"[snowboard] Listening on {}",
			self.acceptor.local_addr().unwrap()
		);

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
	/// function that takes a `Request` and returns a `Future` that resolves to a `Response<'static>`.
	///
	/// The handler function is cloned for each request, and each request is processed in a separate
	/// async task. This means that multiple requests can be processed concurrently.
	///
	/// This function is only available when the `async` feature is enabled.
	///
	/// # Example
	/// ```no_run
	/// use snowboard::{response, Server};
	///
	/// Server::new("localhost:8080").run(async |_| response!(ok));
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

impl Iterator for Server {
	#[cfg(not(feature = "tls"))]
	type Item = (TcpStream, Request);

	#[cfg(feature = "tls")]
	type Item = (TlsStream<TcpStream>, Request);

	fn next(&mut self) -> Option<Self::Item> {
		match self.try_accept() {
			Ok(req) => Some(req),
			Err(e) => {
				if e.kind() != io::ErrorKind::InvalidInput {
					eprintln!("Error: {:?}", e);
					// If the error is not caused by an invalid request, return None,
					// since it is likely a more serious error and will potentialy repeat.
					// (eg. port in use, invalid ssl, etc.)
					return None;
				}

				// Try again
				self.next()
			}
		}
	}
}
