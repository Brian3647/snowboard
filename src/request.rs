use std::net::SocketAddr;
use std::{borrow::Cow, collections::HashMap};

use crate::{Method, Url};

#[cfg(feature = "json")]
use crate::ResponseLike;

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
	/// Body of the request, in bytes.
	/// Use [`Request::text`], [`Request::json`], or [`Request::force_json`]
	/// to get a parsed version of the body.
	pub body: Vec<u8>,
	/// Parsed headers.
	pub headers: HashMap<String, String>,
}

impl Request {
	/// Parses and creates a requeset from raw text and an ip address.
	/// Note that this does not parse the url (See [Request::url]).
	pub fn new(bytes: Vec<u8>, ip: SocketAddr) -> Option<Self> {
		let mut lines = bytes.split(|&byte| byte == b'\n');

		let first_line = String::from_utf8(lines.next()?.to_vec()).ok()?;
		let mut parts = first_line.split_whitespace();

		let method = Method::from(parts.next()?);
		let url = parts.next()?.into();

		// Default capacity for headers is 12, but it will grow automatically if needed.
		let mut headers = HashMap::with_capacity(12);

		let mut in_body = false;
		let mut body = Vec::new();

		for line in lines {
			match (in_body, line == b"\r") {
				(false, true) => in_body = true,
				(true, _) => body.extend_from_slice(line),
				_ => {
					if let Some((key, value)) = Self::parse_header(line) {
						headers.insert(key, value);
					}
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

	fn parse_header(line: &[u8]) -> Option<(String, String)> {
		let pos = line.iter().position(|&byte| byte == b':')?;
		let (key, rest) = line.split_at(pos);
		let value = &rest[1..];

		Some((
			String::from_utf8_lossy(key).trim().to_string(),
			String::from_utf8_lossy(value).trim().to_string(),
		))
	}

	/// Safely gets a header.
	pub fn get_header(&self, key: &str) -> Option<&str> {
		self.headers.get(key).map(|s| s.as_str())
	}

	/// Equivalent to `get_header(key).unwrap_or(default)`
	pub fn get_header_or(&self, key: &str, default: &'static str) -> &str {
		self.get_header(key).unwrap_or(default)
	}

	/// Checks if a header exists.
	pub fn has_header(&self, key: &str) -> bool {
		self.headers.contains_key(key)
	}

	/// Sets a header using any key and value convertible to Strings
	pub fn set_header<T: ToString, K: ToString>(&mut self, k: T, v: K) {
		self.headers.insert(k.to_string(), v.to_string());
	}

	/// Gets the length of the body.
	pub fn len(&self) -> usize {
		self.body.len()
	}

	/// Checks if the body is empty.
	pub fn is_empty(&self) -> bool {
		self.body.is_empty()
	}

	/// Gets the body as a string.
	/// See [`String::from_utf8_lossy`]
	pub fn text(&self) -> Cow<'_, str> {
		String::from_utf8_lossy(&self.body)
	}

	/// Get the body as a JSON value.
	///
	/// This is only intended for custom invalid JSON handling.
	/// Use [`Request::force_json`] to be able to use the `?` operator.
	#[cfg(feature = "json")]
	pub fn json<T>(&self) -> serde_json::Result<T>
	where
		T: for<'a> serde::de::Deserialize<'a>,
	{
		serde_json::from_slice(&self.body)
	}

	/// Get the body as a JSON value, converting a parse error to a bad request response.
	///
	/// # Example
	/// ```rust
	/// # extern crate serde;
	/// # extern crate serde_json;
	/// use snowboard::Server;
	/// use serde::Deserialize;
	///
	/// #[derive(Deserialize)]
	/// struct MyBody {
	/// 	foo: String,
	/// }
	///
	/// fn main() -> snowboard::Result {
	/// 	Server::new("localhost:3000")?.run(|r| {
	/// 		let body: MyBody = r.force_json()?;
	///
	/// 		Ok(serde_json::json!({
	/// 			"foo": body.foo,
	/// 		}))
	/// 	})
	/// }
	/// ```
	#[cfg(feature = "json")]
	pub fn force_json<T>(&self) -> Result<T, crate::Response>
	where
		T: for<'a> serde::de::Deserialize<'a>,
	{
		self.json().map_err(|e| e.to_response())
	}

	/// Get a parsed version of the URL.
	/// See [Url]
	pub fn parse_url(&self) -> Url {
		self.url.as_str().into()
	}

	/// Get the IP address of the client, formatted.
	pub fn pretty_ip(&self) -> String {
		crate::util::format_addr(self.ip)
	}
}
