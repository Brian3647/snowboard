//! `Stream` enum implementation.

use std::io;

use tokio::{
	io::{AsyncReadExt, AsyncWriteExt},
	net::TcpStream,
};
use tokio_native_tls::TlsStream;

/// A stream that can be either secure (https) or not.
pub enum Stream {
	/// A normal TCP stream.
	Normal(TcpStream),
	/// A secure TLS stream.
	Secure(TlsStream<TcpStream>),
}

/// Adds a function to the `Stream` enum.
macro_rules! call_from_inner {
	($name:ident, $desc:literal, $ret:ty) => {
		#[doc = $desc]
		#[inline]
		pub async fn $name(&mut self) -> io::Result<$ret> {
			match self {
				Self::Normal(stream) => stream.$name().await,
				Self::Secure(stream) => stream.$name().await,
			}
		}
	};
	($name:ident, $desc:literal) => {
		call_from_inner!($name, $desc, ());
	};
}

impl Stream {
	/// Writes the given data to the stream.
	#[inline]
	pub async fn write(&mut self, data: &[u8]) -> io::Result<()> {
		match self {
			Self::Normal(stream) => stream.write_all(data).await,
			Self::Secure(stream) => stream.write_all(data).await,
		}
	}

	/// Reads data from the stream to the given buffer.
	#[inline]
	pub async fn read(&mut self, buffer: &mut [u8]) -> io::Result<usize> {
		match self {
			Self::Normal(stream) => stream.read(buffer).await,
			Self::Secure(stream) => stream.read(buffer).await,
		}
	}

	call_from_inner!(shutdown, "Shuts down the stream.");
	call_from_inner!(flush, "Flushes the stream.");
}
