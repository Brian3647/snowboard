//! Server implementation.

pub mod stream;

use std::{
	future::Future,
	io,
	net::{SocketAddr, ToSocketAddrs},
	sync::Arc,
};

use tokio::net::{TcpListener, TcpStream};
use tokio_native_tls::{native_tls, TlsAcceptor};

use crate::{Request, Response, ResponseLike, Stream};

/// Default buffer size for reading requests. (1 KiB)
pub const DEFAULT_BUFFER_SIZE: usize = 1024;

/// Server implementation.
pub struct Server<const BUFFER_SIZE: usize = DEFAULT_BUFFER_SIZE> {
	/// The TLS acceptor, if TLS is enabled.
	tls: Option<TlsAcceptor>,
	/// The addresses the server is listening on.
	addr: SocketAddr,
	/// Whether to insert default headers or not. (true by default)
	insert_default_headers: bool,
	/// WebSocket path and handler.
	ws_handler: Option<(&'static str, fn(crate::WebSocket<'_>))>,
}

impl Server<DEFAULT_BUFFER_SIZE> {
	/// Creates a new server instance using default values.
	/// Note this doesn't actually bind the server to the adress yet.
	///
	/// # Example
	///
	/// ```rust
	/// use snowboard::Server;
	///
	/// let server = Server::from_defaults("localhost:3000").unwrap();
	/// ```
	pub fn from_defaults(addr: impl ToSocketAddrs) -> io::Result<Self> {
		Self::new(addr, true, None, None)
	}
}

impl<const BUFFER_SIZE: usize> Server<BUFFER_SIZE> {
	/// Creates a new server instance. Note this doesn't actually
	/// bind the server to the adress yet.
	///
	/// # Note
	///
	/// Please use `from_defaults` if possible. Both `insert_default_headers`
	/// and `tls` can be updated later using [`Server::insert_default_headers`] and
	/// [`Server::with_tls`]. Same goes for `ws_handler` ([`Server::on_ws`]). The
	/// buffer size can be changed using [`Server::with_buffer_size`].
	pub fn new(
		addr: impl ToSocketAddrs,
		insert_default_headers: bool,
		tls: Option<TlsAcceptor>,
		ws_handler: Option<(&'static str, fn(crate::WebSocket<'_>))>,
	) -> io::Result<Server<BUFFER_SIZE>> {
		Ok(Self {
			tls,
			insert_default_headers,
			addr: addr.to_socket_addrs()?.next().ok_or_else(|| {
				io::Error::new(io::ErrorKind::InvalidInput, "failed to get any adress")
			})?,
			ws_handler,
		})
	}

	/// Sets a TLS acceptor for the server.
	pub fn with_tls(self, acceptor: native_tls::TlsAcceptor) -> Self {
		Self {
			tls: Some(TlsAcceptor::from(acceptor)),
			..self
		}
	}

	/// Changes the buffer size for reading requests.
	pub fn with_buffer_size<const NEW_BUFFER_SIZE: usize>(self) -> Server<NEW_BUFFER_SIZE> {
		Server {
			tls: self.tls,
			addr: self.addr,
			insert_default_headers: self.insert_default_headers,
			ws_handler: self.ws_handler,
		}
	}

	/// Sets whether to insert default headers or not.
	/// Default is true.
	pub fn insert_default_headers(mut self, insert: bool) -> Self {
		self.insert_default_headers = insert;
		self
	}

	/// Returns the address the server is listening on.
	pub fn addr(&self) -> &SocketAddr {
		&self.addr
	}

	/// Returns a pretty string of the address the server is listening on.
	/// Assumes `self.addrs[0]` is the main address.
	pub fn pretty_addr(&self) -> String {
		crate::util::format_addr(&self.addr)
	}

	/// Sets the WebSocket handler.
	/// The handler is called when a WebSocket handshake request is received.
	/// The handler is passed a WebSocket instance, which can be used to
	/// send and receive messages. You might also want to use the `tungstenite`
	/// crate to access to more websocket functionality.
	pub fn on_ws(mut self, path: &'static str, handler: fn(crate::WebSocket<'_>)) -> Self {
		self.ws_handler = Some((path, handler));
		self
	}

	/// Runs the server with the given handler, without
	/// returning if an error occurs. This is only recommended
	/// if your main thread is running the server/you're using it
	/// on a `main` function. Otherwise, use [`Server::checked_run`].
	pub fn run<F, R, Y>(self, handler: F) -> !
	where
		F: Fn(Request) -> R + Send + Sync + 'static,
		R: Future<Output = Y> + Send + 'static,
		Y: ResponseLike + 'static,
	{
		tokio::task::spawn(async move {
			self.checked_run(handler).await.unwrap();
		});
		loop {
			std::thread::park();
		}
	}

	/// Runs the server with the given handler.
	pub async fn checked_run<F, R, Y>(self, handler: F) -> io::Result<()>
	where
		F: Fn(Request) -> R + Send + Sync + 'static,
		R: Future<Output = Y> + Send + 'static,
		Y: ResponseLike + 'static,
	{
		let listener = TcpListener::bind(self.addr).await?;
		let handler_arc = Arc::new(handler);
		let self_arc = Arc::new(self);

		loop {
			let Ok(req) = listener.accept().await else {
				continue;
			};

			let handler = Arc::clone(&handler_arc);
			let self_ = Arc::clone(&self_arc);

			tokio::spawn(async move {
				let _ = self_.handle_raw_tcp(req, handler).await;
			});
		}
	}

	/// Sends a response to the given stream, adding default headers if needed.
	pub async fn send(&self, mut stream: &mut Stream, mut res: Response) -> io::Result<()> {
		if self.insert_default_headers {
			res.with_default_headers().send_to(stream).await
		} else {
			res.send_to(&mut stream).await
		}
	}

	/// Handles a tcp stream.
	pub async fn handle_raw_tcp<F, R, Y>(
		&self,
		req: (TcpStream, SocketAddr),
		handler: Arc<F>,
	) -> io::Result<()>
	where
		F: Fn(Request) -> R,
		R: Future<Output = Y>,
		Y: ResponseLike,
	{
		let mut stream = if let Some(tls) = &self.tls {
			match tls.accept(req.0).await {
				Ok(stream) => Stream::Secure(stream),
				Err(_) => {
					return Err(io::Error::new(
						io::ErrorKind::InvalidData,
						"failed to accept TLS connection",
					))
				}
			}
		} else {
			Stream::Normal(req.0)
		};

		while let Ok(true) = self.handle_stream((&mut stream, req.1), &handler).await {}

		stream.flush().await
	}

	/// Reads a request from the given stream and sends a response.
	/// If the connection is keep-alive, the function will return `Ok(true)`.
	/// Otherwise, it will return `Ok(false)`. This should be run in a loop,
	/// breaking when `false` is returned.
	pub async fn handle_stream<F, R, Y>(
		&self,
		req: (&mut Stream, SocketAddr),
		handler: &Arc<F>,
	) -> io::Result<bool>
	where
		F: Fn(Request) -> R,
		R: Future<Output = Y>,
		Y: ResponseLike,
	{
		let (mut stream, addr) = req;
		let mut buffer = [0u8; BUFFER_SIZE];
		let read = stream.read(&mut buffer).await?;
		let bytes = &buffer[..read];

		let Some(mut req) = Request::new(&bytes, addr) else {
			let mut res = Response::bad_request("Failed to parse request".into(), None);
			res.send_to(&mut stream).await?;
			return Ok(false);
		};

		if crate::ws::maybe_websocket(self.ws_handler, &mut stream, &mut req).await {
			return Ok(false); // WebSocket handled
		}

		let should_continue = match req.get_header("connection") {
			Some("close") | None => false,
			_ => true, // keep-alive
		};

		let res = handler(req).await.to_response();
		self.send(stream, res).await?;
		Ok(should_continue)
	}
}
