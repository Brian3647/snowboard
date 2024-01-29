//! Server implementation.

pub mod stream;

use std::{
	future::Future,
	io,
	net::{SocketAddr, ToSocketAddrs},
	sync::Arc,
};

use tokio::net::{TcpListener, TcpStream};

use tokio_native_tls::TlsAcceptor;

use crate::{Request, Response, ResponseLike};

use self::stream::Stream;

/// Default buffer size for reading requests. (4 KiB)
pub const DEFAULT_BUFFER_SIZE: usize = 1024 * 4;

/// Server implementation.
pub struct Server<const BUFFER_SIZE: usize = DEFAULT_BUFFER_SIZE> {
	/// The TLS acceptor, if TLS is enabled.
	tls: Option<TlsAcceptor>,
	/// The addresses the server is listening on.
	addr: SocketAddr,
	/// Whether to insert default headers or not. (true by default)
	insert_default_headers: bool,
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
		Self::new(addr, true, None)
	}
}

impl<const BUFFER_SIZE: usize> Server<BUFFER_SIZE> {
	/// Creates a new server instance. Note this doesn't actually
	/// bind the server to the adress yet.
	///
	/// # Note
	///
	/// Please use `from_defaults` if you aren't manually specifying
	/// the buffer size. Both `insert_default_headers` and `tls` can
	/// be updated later using [`Server::insert_default_headers`] and
	/// [`Server::with_tls`].
	pub fn new(
		addr: impl ToSocketAddrs,
		insert_default_headers: bool,
		tls: Option<TlsAcceptor>,
	) -> io::Result<Server<BUFFER_SIZE>> {
		Ok(Self {
			tls,
			insert_default_headers,
			addr: addr.to_socket_addrs()?.next().ok_or_else(|| {
				io::Error::new(io::ErrorKind::InvalidInput, "failed to get any adress")
			})?,
		})
	}

	/// Sets a TLS acceptor for the server.
	pub fn with_tls(self, acceptor: TlsAcceptor) -> Self {
		Self {
			tls: Some(acceptor),
			..self
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
		tokio::task::spawn_local(async move {
			self.checked_run(handler).await.unwrap();
		});
		unreachable!("Server shouldn't stop running")
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

			{
				let handler = Arc::clone(&handler_arc);
				let self_ = Arc::clone(&self_arc);

				tokio::spawn(async move {
					let _ = self_.handle_stream(req, handler).await;
				});
			}
		}
	}

	/// Sends a response to the given stream, adding default headers if needed.
	async fn send(&self, mut stream: Stream, mut res: Response) -> io::Result<()> {
		if self.insert_default_headers {
			res.with_default_headers().send_to(&mut stream).await
		} else {
			res.send_to(&mut stream).await
		}
	}

	/// Handles a stream.
	pub async fn handle_stream<F, R, Y>(
		&self,
		req: (TcpStream, SocketAddr),
		handler: Arc<F>,
	) -> io::Result<()>
	where
		F: Fn(Request) -> R,
		R: Future<Output = Y>,
		Y: ResponseLike,
	{
		let (stream, addr) = req;
		let mut stream = if let Some(tls) = &self.tls {
			match tls.accept(stream).await {
				Ok(stream) => Stream::Secure(stream),
				Err(_) => {
					return Err(io::Error::new(
						io::ErrorKind::InvalidData,
						"failed to accept TLS connection",
					))
				}
			}
		} else {
			Stream::Normal(stream)
		};

		let mut buffer = [0u8; BUFFER_SIZE];
		let read = stream.read(&mut buffer).await?;
		let bytes = &buffer[..read];
		let req = match Request::new(bytes, addr) {
			Some(req) => req,
			None => {
				let mut res = Response::bad_request("Failed to parse request".into(), None);
				res.send_to(&mut stream).await?;
				return Ok(());
			}
		};

		let res = handler(req).await.to_response();
		self.send(stream, res).await?;
		Ok(())
	}
}
