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
	pub fn new(text: String, ip: SocketAddr) -> Option<Self> {
		let mut lines = text.lines();

		let first_line = lines.next()?;
		let mut parts = first_line.split_whitespace();

		let method = Method::from(parts.next()?.to_string());
		let url = parts.next()?.into();

		let mut headers = HashMap::new();
		let mut in_body = false;
		let mut body = String::new();

		for line in lines {
			if line.is_empty() {
				in_body = true;
				continue;
			} else if in_body {
				body.push_str(line);
				continue;
			}

			let mut parts = line.splitn(2, ':');
			let key = parts.next()?.into();
			let value = parts.next()?.trim().into();

			headers.insert(key, value);
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
	pub fn set_header<T, K>(&mut self, key: T, value: K)
	where
		T: Into<String>,
		K: Into<String>,
	{
		self.headers.insert(key.into(), value.into());
	}

	/// Get a parsed version of the URL.
	/// See [Url]
	pub fn parse_url(&self) -> Url {
		self.url.as_str().into()
	}
}

impl Default for Request {
	fn default() -> Self {
		Self {
			ip: SocketAddr::new([0, 0, 0, 0].into(), 80),
			url: "/".into(),
			method: Method::GET,
			body: String::new(),
			headers: HashMap::new(),
		}
	}
}
