use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Method {
	GET,
	POST,
	PUT,
	DELETE,
	HEAD,
	OPTIONS,
	CONNECT,
	PATCH,
	TRACE,
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

impl From<String> for Method {
	fn from(method: String) -> Self {
		match method.as_str() {
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HttpVersion {
	V1_0,
	V1_1,
	V2_0,
	V3_0,
	UNKNOWN,
}

impl Display for HttpVersion {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let version = match self {
			HttpVersion::V1_0 => "1.0",
			HttpVersion::V1_1 => "1.1",
			HttpVersion::V2_0 => "2.0",
			HttpVersion::V3_0 => "3.0",
			HttpVersion::UNKNOWN => "1.1",
		};

		write!(f, "HTTP/{}", version)
	}
}

impl From<String> for HttpVersion {
	fn from(version: String) -> Self {
		match version.to_uppercase().as_str() {
			"HTTP/1.0" => HttpVersion::V1_0,
			"HTTP/1.1" => HttpVersion::V1_1,
			"HTTP/2.0" => HttpVersion::V2_0,
			"HTTP/3.0" => HttpVersion::V3_0,
			_ => HttpVersion::UNKNOWN,
		}
	}
}
