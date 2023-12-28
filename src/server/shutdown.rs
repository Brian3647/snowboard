//! Shutdown trait

use std::io;

use mio::net as mio_net;

/// A trait for shutting down a stream.
pub trait Shutdown {
	/// Shutdown the stream.
	fn shutdown_stream(&mut self) -> io::Result<()>;
}

impl Shutdown for mio_net::TcpStream {
	#[inline(always)]
	fn shutdown_stream(&mut self) -> io::Result<()> {
		self.shutdown(std::net::Shutdown::Both)
	}
}

impl Shutdown for &mut mio_net::TcpStream {
	#[inline(always)]
	fn shutdown_stream(&mut self) -> io::Result<()> {
		self.shutdown(std::net::Shutdown::Both)
	}
}

#[cfg(feature = "tls")]
impl Shutdown for native_tls::TlsStream<mio_net::TcpStream> {
	#[inline(always)]
	fn shutdown_stream(&mut self) -> io::Result<()> {
		self.shutdown()
	}
}

#[cfg(feature = "tls")]
impl Shutdown for &mut native_tls::TlsStream<mio_net::TcpStream> {
	#[inline(always)]
	fn shutdown_stream(&mut self) -> io::Result<()> {
		self.shutdown()
	}
}
