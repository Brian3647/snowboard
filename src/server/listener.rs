//! Listener implementation.

use std::{io, net, time::Duration};

use mio::{net as mio_net, Events, Interest, Poll, Token};

/// Mio token for the server.
const SERVER: Token = Token(0);

/// A wrapper around `TcpListener` that allows faster request handling.
pub struct Listener {
	/// The inner `TcpListener` instance.
	inner: mio_net::TcpListener,
}

impl Listener {
	/// Create a new listener instance from the given address.
	pub fn new(addr: net::SocketAddr) -> io::Result<Self> {
		let inner = mio_net::TcpListener::bind(addr)?;

		Ok(Self { inner })
	}

	/// Run the listener in a loop.
	pub fn r#loop<F: Fn(mio_net::TcpStream, net::SocketAddr)>(
		&mut self,
		handler: F,
	) -> io::Result<()> {
		let mut poll = Poll::new()?;
		let mut events = Events::with_capacity(128);
		poll.registry().register(
			&mut self.inner,
			SERVER,
			Interest::READABLE | Interest::WRITABLE,
		)?;

		loop {
			self.loop_inner(&mut poll, &mut events, &handler)?;
		}
	}

	/// Inner part of the loop, separated to a function for testing & identation.
	#[inline(always)]
	fn loop_inner<F: Fn(mio_net::TcpStream, net::SocketAddr)>(
		&self,
		poll: &mut Poll,
		events: &mut Events,
		handler: &F,
	) -> io::Result<()> {
		poll.poll(events, Some(Duration::from_millis(100)))?;

		for event in events.iter() {
			if event.token() != SERVER || !event.is_readable() {
				continue;
			}

			match self.inner.accept() {
				Ok((connection, address)) => handler(connection, address),
				Err(ref e) if is_wouldblock(e) => break,
				Err(e) => return Err(e),
			}
		}

		Ok(())
	}
}

/// Checks if the error is a wouldblock error.
#[inline(always)]
fn is_wouldblock(e: &io::Error) -> bool {
	e.kind() == io::ErrorKind::WouldBlock
}
