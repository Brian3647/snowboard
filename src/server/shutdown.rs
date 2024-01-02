//! Shutdown trait

use async_std::{io, net};

/// A trait for shutting down a stream.
pub trait Shutdown {
	/// Shutdown the stream.
	fn shutdown_stream(&mut self) -> io::Result<()>;
}

impl Shutdown for net::TcpStream {
	#[inline(always)]
	fn shutdown_stream(&mut self) -> io::Result<()> {
		self.shutdown(std::net::Shutdown::Both)
	}
}

impl Shutdown for &mut net::TcpStream {
	#[inline(always)]
	fn shutdown_stream(&mut self) -> io::Result<()> {
		self.shutdown(std::net::Shutdown::Both)
	}
}

#[cfg(feature = "tls")]
impl Shutdown for async_native_tls::TlsStream<net::TcpStream> {
	#[inline(always)]
	fn shutdown_stream(&mut self) -> io::Result<()> {
		Ok(())
	}
}

#[cfg(feature = "tls")]
impl Shutdown for &mut async_native_tls::TlsStream<net::TcpStream> {
	#[inline(always)]
	fn shutdown_stream(&mut self) -> io::Result<()> {
		Ok(())
	}
}
