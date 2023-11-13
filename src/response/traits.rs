use super::Response;

/// A trait for everything that can be converted into a Response.
pub trait ResponseLike {
	/// Converts `self` into a `Response`.
	fn to_response(self) -> Response;
}

impl ResponseLike for Response {
	fn to_response(self) -> Response {
		self
	}
}

impl ResponseLike for () {
	#[inline]
	fn to_response(self) -> Response {
		Response::default()
	}
}

impl ResponseLike for &str {
	#[inline]
	fn to_response(self) -> Response {
		crate::response!(ok, self)
	}
}

impl ResponseLike for String {
	#[inline]
	fn to_response(self) -> Response {
		crate::response!(ok, self)
	}
}

impl ResponseLike for Vec<u8> {
	#[inline]
	fn to_response(self) -> Response {
		let mut res = crate::response!(ok);
		res.set_bytes(&self);
		res
	}
}

// Particuraly useful for `?` operators when using outside functions.
impl ResponseLike for Result<Response, Response> {
	#[inline]
	fn to_response(self) -> Response {
		match self {
			Ok(res) => res,
			Err(res) => res,
		}
	}
}
