use std::net::SocketAddr;
use std::{borrow::Cow, collections::HashMap};

use crate::{Method, Url};

/// A server request.
/// Parses the raw request string into a more usable format.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "json", derive(serde::Serialize))]
pub struct Request {
	/// The ip from the socket connection.
	pub ip: SocketAddr,
	/// Raw URL string.
	/// Use `Request::parse_url()` to get a parsed version of the URL
	pub url: String,
	/// Method used in the request. Might be Method::Unknown if parsing fails.
	pub method: Method,
	/// Body of the request.
	pub body: Vec<u8>,
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

		let method = Method::from(parts.next()?);
		let url = parts.next()?.into();

		// Default capacity for headers is 12, but it will grow autoamtically if needed.
		let mut headers = HashMap::with_capacity(12);

		let mut in_body = false;
		let mut body = Vec::default();

		for line in lines {
			match (in_body, line.is_empty()) {
				(false, true) => in_body = true,
				(true, _) => body.extend_from_slice(line.as_bytes()),
				_ => {
					let parts = line.split_once(':')?;
					let key = parts.0.into();
					let value = parts.1.trim().into();

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

	/// Gets the body as a string.
	/// See [std::string::String::from_utf8]
	pub fn text(&self) -> Cow<'_, str> {
		String::from_utf8_lossy(&self.body)
	}

	/// Get the body as a JSON value.
	#[cfg(feature = "json")]
	pub fn json<T>(&self) -> serde_json::Result<T>
	where
		T: for<'a> serde::de::Deserialize<'a>,
	{
		serde_json::from_slice(&self.body)
	}

	/// Get a parsed version of the URL.
	/// See [Url]
	pub fn parse_url(&self) -> Url {
		self.url.as_str().into()
	}
}
