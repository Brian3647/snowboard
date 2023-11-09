use std::collections::HashMap;
use std::net::SocketAddr;

use crate::{Method, Url};

/// A server request.
/// Parses the raw request string into a more usable format.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Request {
	/// The ip from the socket connection.
	pub ip: SocketAddr,
	/// Raw URL string.
	/// Use `Request::parse_url()` to get a parsed version of the URL
	pub url: String,
	/// Method used in the request. Might be Method::Unknown if parsing fails.
	pub method: Method,
	/// Body of the request.
	pub body: String,
	/// Parsed headers.
	pub headers: HashMap<String, String>,
}

impl Request {
	/// Parses and creates a requeset from raw text and an ip address.
	/// Note that this does not parse the url (See [Request::url]).
	pub fn new(text: impl ToString, ip: SocketAddr) -> Option<Self> {
		let text = text.to_string();
		let mut lines = text.lines();

		let first_line = lines.next()?;
		let mut parts = first_line.split_whitespace();

		let method = Method::from(parts.next()?.to_string());
		let url = parts.next()?.into();

		// Default capacity for headers is 12, but it will grow autoamtically if needed.
		let mut headers = HashMap::with_capacity(12);

		let mut in_body = false;
		let mut body = String::default();

		for line in lines {
			match (in_body, line.is_empty()) {
				(false, true) => in_body = true,
				(true, _) => body.push_str(line),
				_ => {
					let parts = line.split_once(':')?;
					let key = parts.0.into();
					let value = parts.0.trim().into();

					headers.insert(key, value);
				}
			}
		}

		Some(Self {
			ip,
			url,
			method,
			body,
			headers,
		})
	}

	/// Safely gets a header.
	pub fn get_header(&self, key: &str) -> Option<&str> {
		self.headers.get(key).map(|s| s.as_str())
	}

	/// Equivalent to get_header(key).unwrap_or(default)
	pub fn get_header_or(&self, key: &str, default: &'static str) -> &str {
		self.get_header(key).unwrap_or(default)
	}

	/// Sets a header using any key and value convertible to Strings
	pub fn set_header<T: ToString, K: ToString>(&mut self, k: T, v: K) {
		self.headers.insert(k.to_string(), v.to_string());
	}

	/// Get a parsed version of the URL.
	/// See [Url]
	pub fn parse_url(&self) -> Url {
		self.url.as_str().into()
	}
}
