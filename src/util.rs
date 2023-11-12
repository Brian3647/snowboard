use std::fmt::Display;

/// Any valid HTTP method.
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
		let method = match self {
			Method::GET => "GET",
			Method::POST => "POST",
			Method::PUT => "PUT",
			Method::DELETE => "DELETE",
			Method::HEAD => "HEAD",
			Method::OPTIONS => "OPTIONS",
			Method::CONNECT => "CONNECT",
			Method::PATCH => "PATCH",
			Method::TRACE => "TRACE",
			Method::UNKNOWN => "UNKNOWN",
		};

		write!(f, "{}", method)
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
		let version = match self {
			HttpVersion::V1_0 => "1.0",
			HttpVersion::V1_1 => "1.1",
			HttpVersion::V2_0 => "2.0",
			HttpVersion::V3_0 => "3.0",
			// If the version isn't valid, and the user tries to send a response,
			// it'll just send a 1.1 response. This might cause problems, but it's
			// better than crashing.
			#[allow(clippy::print_in_format_impl)]
			HttpVersion::UNKNOWN => {
				eprintln!("Warning: Unknown HTTP version. Defaulting to 1.1");
				"1.1"
			}
		};

		write!(f, "HTTP/{}", version)
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
