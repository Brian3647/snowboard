//! A module that provides code to handle the HTTP/HTTPS header method types.

use std::{fmt::Display, net::SocketAddr};

/// Any valid HTTP method.
#[cfg_attr(feature = "json", derive(serde::Serialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Method {
	/// GET
	GET,
	/// POST
	POST,
	/// PUT
	PUT,
	/// DELETE
	DELETE,
	/// HEAD
	HEAD,
	/// OPTIONS
	OPTIONS,
	/// CONNECT
	CONNECT,
	/// PATCH
	PATCH,
	/// TRACE
	TRACE,
	/// Unknown method
	UNKNOWN,
}

impl Display for Method {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{:?}", self)
	}
}

impl From<&str> for Method {
	fn from(method: &str) -> Self {
		match method {
			"GET" => Method::GET,
			"POST" => Method::POST,
			"PUT" => Method::PUT,
			"DELETE" => Method::DELETE,
			"HEAD" => Method::HEAD,
			"OPTIONS" => Method::OPTIONS,
			"CONNECT" => Method::CONNECT,
			"PATCH" => Method::PATCH,
			"TRACE" => Method::TRACE,
			_ => Method::UNKNOWN,
		}
	}
}

/// HTTP protocol version.
/// `HttpVersion::UNKNOWN` is used when the version is not specified or not valid.
#[cfg_attr(feature = "json", derive(serde::Serialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HttpVersion {
	/// HTTP/1.0
	V1_0,
	/// HTTP/1.1
	V1_1,
	/// HTTP/2.0
	V2_0,
	/// HTTP/3.0
	V3_0,
	/// Unknown version
	UNKNOWN,
}

impl Display for HttpVersion {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(
			f,
			"HTTP/{}",
			match self {
				HttpVersion::V1_0 => "1.0",
				HttpVersion::V1_1 => "1.1",
				HttpVersion::V2_0 => "2.0",
				HttpVersion::V3_0 => "3.0",
				// If the version isn't valid, and the user tries to send a response,
				// it'll just send a 1.1 response. This might cause problems, but it's
				// better than crashing.
				HttpVersion::UNKNOWN => "1.1",
			}
		)
	}
}

impl From<&str> for HttpVersion {
	fn from(version: &str) -> Self {
		match version {
			"HTTP/1.0" => HttpVersion::V1_0,
			"HTTP/1.1" => HttpVersion::V1_1,
			"HTTP/2.0" => HttpVersion::V2_0,
			"HTTP/3.0" => HttpVersion::V3_0,
			_ => HttpVersion::UNKNOWN,
		}
	}
}

/// Formats a socket address into something usable.
pub fn format_addr(addr: SocketAddr) -> String {
	match addr {
		SocketAddr::V4(v4) => {
			if v4.ip().is_loopback() {
				format!("localhost:{}", v4.port())
			} else {
				v4.to_string()
			}
		}
		SocketAddr::V6(v6) => {
			if v6.ip().is_loopback() {
				format!("localhost:{}", v6.port())
			} else {
				format!("{}:{}", v6.ip(), v6.port())
			}
		}
	}
}
