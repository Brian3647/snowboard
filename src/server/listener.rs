//! Listener implementation.

use futures::stream::StreamExt;
use std::future::Future;
use std::io;

use async_std::net::TcpListener;
use async_std::net::{self, ToSocketAddrs};

/// A wrapper around `TcpListener` that allows faster request handling.
pub struct Listener {
	/// The inner `TcpListener` instance.
	inner: TcpListener,
}

impl Listener {
	/// Create a new listener instance from the given address.
	pub async fn new<A: ToSocketAddrs>(addr: A) -> io::Result<Self> {
		let inner = TcpListener::bind(addr).await?;

		Ok(Self { inner })
	}

	/// Run the listener in a loop.
	pub async fn r#loop<F: Fn(net::TcpStream, net::SocketAddr) -> T, T: Future<Output = ()>>(
		&self,
		handler: &F,
	) -> io::Result<()> {
		self.inner
			.incoming()
			.for_each_concurrent(None, move |stream| async move {
				if let Ok(stream) = stream {
					if let Ok(peer_addr) = stream.peer_addr() {
						handler(stream, peer_addr).await
					}
				}
			})
			.await;

		Ok(())
	}
}
