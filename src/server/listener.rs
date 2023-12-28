//! Listener implementation.

use std::{io, net};

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
		poll.registry()
			.register(&mut self.inner, SERVER, Interest::READABLE)?;

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
		poll.poll(events, None)?;

		for event in events.iter() {
			match event.token() {
				SERVER => {
					let (stream, addr) = match self.inner.accept() {
						Ok((stream, addr)) => (stream, addr),
						Err(e) if is_wouldblock(&e) => continue,
						Err(e) => return Err(e),
					};

					handler(stream, addr);
				}
				_ => unreachable!(),
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
