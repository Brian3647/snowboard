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

impl Stream {
	/// Writes the given data to the stream.
	#[inline]
	pub async fn write(&mut self, data: &[u8]) -> io::Result<()> {
		match self {
			Self::Normal(stream) => stream.write_all(data).await,
			Self::Secure(stream) => stream.write_all(data).await,
		}
	}

	/// Flushes the stream.
	#[inline]
	pub async fn flush(&mut self) -> io::Result<()> {
		match self {
			Self::Normal(stream) => stream.flush().await,
			Self::Secure(stream) => stream.flush().await,
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
}
