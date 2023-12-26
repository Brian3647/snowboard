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
/// // Response with body, headers and custom HTTP version.
/// let body = "everything's fine!";
/// let headers = headers! {
///     "Content-Type" => "text/html",
///     "X-Hello" => "World!",
///     "X-Number" => 42,
/// };
/// let response = response!(ok, body, headers, HttpVersion::V1_0);
/// ```
///
/// See [headers!](crate::headers) for more information about the headers macro.
#[macro_export]
macro_rules! response {
	(ok) => {
		$crate::Response::default()
	};

	($type:ident) => {
		$crate::Response::$type(vec![], None, $crate::DEFAULT_HTTP_VERSION)
	};

	($type:ident,$body:expr) => {
		$crate::Response::$type($body.into(), None, $crate::DEFAULT_HTTP_VERSION)
	};

	($type:ident,$body:expr,$headers:expr) => {
		$crate::Response::$type($body.into(), Some($headers), $crate::DEFAULT_HTTP_VERSION)
	};

	($type:ident,$body:expr,$headers:expr,$http_version:expr) => {
		$crate::Response::$type($body.into(), Some($headers), $http_version)
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
