mod response_types;
mod traits;

pub use traits::ResponseLike;

use std::{collections::HashMap, fmt, io};

use crate::HttpVersion;

/// The default HTTP version used by the server.
pub const DEFAULT_HTTP_VERSION: HttpVersion = HttpVersion::V1_1;

/// Response struct.
/// Contains the response data and converts it to text if needed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Response {
	/// HTTP protocol version.
	pub version: HttpVersion,
	/// HTTP status code.
	pub status: u16,
	/// According text for the status.
	pub status_text: &'static str,
	/// The request body, stored in bytes.
	pub bytes: Vec<u8>,
	/// Headers of the response
	pub headers: Option<Headers>,
}

/// Equivalent to `HashMap<&'static str, String>`.
pub type Headers = HashMap<&'static str, String>;

impl Response {
	/// Manually create a Response instance.
	/// Use Response::ok(), Response::bad_request() etc. instead when possible.
	pub fn new(
		version: HttpVersion,
		status: u16,
		status_text: &'static str,
		bytes: Vec<u8>,
		headers: Option<Headers>,
	) -> Self {
		Self {
			version,
			status,
			status_text,
			bytes,
			headers,
		}
	}

	/// Writes the response to a TcpStream.
	pub fn send_to<T: io::Write>(&mut self, stream: &mut T) -> Result<(), io::Error> {
		let mut first_bytes = self.prepare_response().into_bytes();
		first_bytes.append(&mut self.bytes);
		stream.write_all(&first_bytes)?;
		stream.flush()
	}

	/// Sets a header to the response. Use Response::content_type for the 'Content-Type' header.
	pub fn set_header(&mut self, key: &'static str, value: String) {
		if let Some(headers) = &mut self.headers {
			headers.insert(key, value);
		} else {
			let mut headers = HashMap::new();
			headers.insert(key, value);
			self.headers = Some(headers);
		}
	}

	/// Sets the content type of the response. Note that this does not check if the content type
	/// is valid, so be careful.
	#[inline]
	pub fn content_type(&mut self, value: String) {
		self.set_header("Content-Type", value)
	}

	/// Returns the first lines of the generated response.
	/// This function is used internally to create the response.
	/// If you want to get the full response, use `Display` instead.
	/// Only the body is missing from the response.
	fn prepare_response(&self) -> String {
		let mut text = format!("{} {} {}\r\n", self.version, self.status, self.status_text);

		if let Some(headers) = &self.headers {
			for (key, value) in headers {
				text.push_str(&format!("{key}: {value}\r\n"));
			}
		}

		text += "\r\n";
		text
	}
}

impl fmt::Display for Response {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let mut text = self.prepare_response();
		text += String::from_utf8_lossy(&self.bytes).as_ref();
		write!(f, "{}", text)
	}
}

impl Default for Response {
	fn default() -> Self {
		Self {
			version: DEFAULT_HTTP_VERSION,
			status: 200,
			status_text: "Ok",
			bytes: vec![],
			headers: None,
		}
	}
}
