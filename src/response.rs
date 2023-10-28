// Why is this file so big?
// This file includes the functions for each response type.
// (eg. ok, not_found, etc.)

use std::{
	collections::HashMap,
	fmt::Display,
	io::{self, Write},
};

use crate::HttpVersion;

const DEFAULT_HTTP_VERSION: HttpVersion = HttpVersion::V1_1;

/// Response struct.
/// Contains the response data and converts it to text if needed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Response {
	pub version: HttpVersion,
	pub status: u16,
	pub status_text: &'static str,
	pub body: String,
	pub headers: HashMap<&'static str, &'static str>,
}

type OptHeaders = Option<HashMap<&'static str, &'static str>>;

impl Response {
	pub fn new(
		version: HttpVersion,
		status: u16,
		status_text: &'static str,
		body: String,
		headers: HashMap<&'static str, &'static str>,
	) -> Self {
		Self {
			version,
			status,
			status_text,
			body,
			headers,
		}
	}

	pub fn send_to(&self, stream: &mut std::net::TcpStream) -> Result<(), io::Error> {
		let text = self.to_string();
		let bytes = text.as_bytes();

		stream.write_all(bytes)?;
		stream.flush()?;
		Ok(())
	}

	pub fn set_header(&mut self, key: &'static str, value: &'static str) {
		self.headers.insert(key, value);
	}

	pub fn content_type(&mut self, value: &'static str) {
		self.headers.insert("Content-Type", value);
	}

	/// Mostly used internally to create response functions during compile time.
	/// If possible, use the `response!` macro instead or Response::new().
	pub fn create_response(
		http_version: Option<HttpVersion>,
		code: u16,
		text: &'static str,
		body: Option<String>,
		headers: OptHeaders,
	) -> Self {
		Self::new(
			http_version.unwrap_or(DEFAULT_HTTP_VERSION),
			code,
			text,
			body.unwrap_or_default(),
			headers.unwrap_or_default(),
		)
	}
}

impl Display for Response {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let mut text = format!("{} {} {}\r\n", self.version, self.status, self.status_text);

		for (key, value) in &self.headers {
			text += &format!("{key}: {value}\r\n");
		}

		text += "\r\n";
		text += &self.body;

		write!(f, "{}", text)
	}
}

impl Default for Response {
	fn default() -> Self {
		Self {
			version: HttpVersion::V1_1,
			status: 200,
			status_text: "OK",
			body: String::new(),
			headers: HashMap::new(),
		}
	}
}

/// A quick way to create responses.
///
/// Usage:
/// ```
/// use snowboard::{response, HttpVersion};
/// use std::collections::HashMap;
///
/// // Response with no headers and no body.
/// let response = response!(bad_request);
///
/// // Response with body and no headers.
/// // Note that $body requires to implement Display.
/// let response =  response!(internal_server_error, "oopsies");
///
/// // Response with body, headers and custom HTTP version.
/// let body = "everything's fine!";
/// let mut headers = HashMap::new();
/// headers.insert("X-Hello", "World!");
/// let response = response!(ok, body, headers, HttpVersion::V1_0);
/// ```
#[macro_export]
macro_rules! response {
	(ok) => {
		$crate::Response::default()
	};

	($type:ident) => {
		$crate::Response::$type(None, None, None)
	};

	($type:ident,$body:expr) => {
		$crate::Response::$type(Some($body.to_string()), None, None)
	};

	($type:ident,$body:expr,$headers:expr) => {
		$crate::Response::$type(Some($body.to_string()), Some($headers), None)
	};

	($type:ident,$body:expr,$headers:expr,$http_version:expr) => {
		$crate::Response::$type(Some($body.to_string()), Some($headers), Some($http_version))
	};
}

// Macro rule used to create response types during compile time.
macro_rules! create_response_types {
    ($($name:ident, $code:expr, $text:expr);*) => {
        impl Response {
        $(
            #[inline(always)]
            pub fn $name(body: Option<String>, headers: OptHeaders, http_version: Option<HttpVersion>) -> Self {
                Self::create_response(http_version, $code, $text, body, headers)
            }
        )*
        }
    };
}

create_response_types!(
	continue_, 100, "Continue";
	switching_protocols, 101, "Switching Protocols";
	processing, 102, "Processing";
	early_hints, 103, "Early Hints";
	ok, 200, "Ok";
	created, 201, "Created";
	accepted, 202, "Accepted";
	non_authoritative_information, 203, "Non-Authoritative Information";
	no_content, 204, "No Content";
	reset_content, 205, "Reset Content";
	partial_content, 206, "Partial Content";
	multi_status, 207, "Multi-Status";
	already_reported, 208, "Already Reported";
	im_used, 226, "IM Used";
	multiple_choices, 300, "Multiple Choices";
	moved_permanently, 301, "Moved Permanently";
	found, 302, "Found";
	see_other, 303, "See Other";
	not_modified, 304, "Not Modified";
	use_proxy, 305, "Use Proxy";
	temporary_redirect, 307, "Temporary Redirect";
	permanent_redirect, 308, "Permanent Redirect";
	bad_request, 400, "Bad Request";
	unauthorized, 401, "Unauthorized";
	payment_required, 402, "Payment Required";
	forbidden, 403, "Forbidden";
	not_found, 404, "Not Found";
	method_not_allowed, 405, "Method Not Allowed";
	not_acceptable, 406, "Not Acceptable";
	proxy_authentication_required, 407, "Proxy Authentication Required";
	request_timeout, 408, "Request Timeout";
	conflict, 409, "Conflict";
	gone, 410, "Gone";
	length_required, 411, "Length Required";
	precondition_failed, 412, "Precondition Failed";
	payload_too_large, 413, "Payload Too Large";
	uri_too_long, 414, "URI Too Long";
	unsupported_media_type, 415, "Unsupported Media Type";
	range_not_satisfiable, 416, "Range Not Satisfiable";
	expectation_failed, 417, "Expectation Failed";
	im_a_teapot, 418, "I'm a teapot";
	misdirected_request, 421, "Misdirected Request";
	unprocessable_entity, 422, "Unprocessable Entity";
	locked, 423, "Locked";
	failed_dependency, 424, "Failed Dependency";
	too_early, 425, "Too Early";
	upgrade_required, 426, "Upgrade Required";
	precondition_required, 428, "Precondition Required";
	too_many_requests, 429, "Too Many Requests";
	request_header_fields_too_large, 431, "Request Header Fields Too Large";
	unavailable_for_legal_reasons, 451, "Unavailable For Legal Reasons";
	internal_server_error, 500, "Internal Server Error";
	not_implemented, 501, "Not Implemented";
	bad_gateway, 502, "Bad Gateway";
	service_unavailable, 503, "Service Unavailable";
	gateway_timeout, 504, "Gateway Timeout";
	http_version_not_supported, 505, "HTTP Version Not Supported";
	variant_also_negotiates, 506, "Variant Also Negotiates";
	insufficient_storage, 507, "Insufficient Storage";
	loop_detected, 508, "Loop Detected";
	not_extended, 510, "Not Extended";
	network_authentication_required, 511, "Network Authentication Required"
);
