//! A module that handles and provides easy to use macros for the user.

/// A quick way to create responses.
///
/// Usage:
/// ```
/// use snowboard::{response, HttpVersion, headers};
/// use std::collections::HashMap;
///
/// // Response with no headers and no body.
/// let response = response!(bad_request);
///
/// // Response with body and no headers.
/// // Note that $body requires to implement `Into<Vec<u8>>`.
/// let response =  response!(internal_server_error, "oopsies");
///
/// // Response with body, & headers.
/// let body = "everything's fine!";
/// let headers = headers! {
///     "Content-Type" => "text/html",
///     "X-Hello" => "World!",
///     "X-Number" => 42,
/// };
/// let response = response!(ok, body, headers);
/// ```
///
/// See [headers!](crate::headers) for more information about the headers macro.
#[macro_export]
macro_rules! response {
	(ok) => {
		::snowboard::Response::default()
	};

	($type:ident) => {
		::snowboard::Response::$type(vec![], None)
	};

	($type:ident,$body:expr) => {
		::snowboard::Response::$type($body.into(), None)
	};

	($type:ident,$body:expr,$headers:expr) => {
		::snowboard::Response::$type($body.into(), Some($headers))
	};
}

/// A quick way to create a header HashMap.
///
/// A similar version of this macro can be found in other
/// crates as `map!` or `hashmap!`.
///
/// This will convert any `$value` to a String, since
/// the headers are stored as `HashMap<&str, String>`.
///
/// Example:
/// ```rust
/// use snowboard::headers;
///
/// let headers = headers! {
///     "Content-Type" => "text/html",
///     "X-Hello" => "World!",
///     "X-Number" => 42,
/// };
/// ```
#[macro_export]
macro_rules! headers {
	($($name:expr => $value:expr $(,)?)*) => {{
		let mut map = ::std::collections::HashMap::<&str, String>::new();
		$(map.insert($name, $value.to_string());)*
		map
	}};
}
