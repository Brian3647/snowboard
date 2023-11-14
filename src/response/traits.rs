use super::Response;

/// A trait for everything that can be converted into a Response.
pub trait ResponseLike {
	/// Converts `self` into a `Response`.
	fn to_response(self) -> Response;
}

impl ResponseLike for Response {
	#[inline]
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
		crate::response!(ok, self)
	}
}

// Particuraly useful for `?` operators when using outside functions.
impl<T> ResponseLike for Result<T, Response>
where
	T: ResponseLike,
{
	fn to_response(self) -> Response {
		match self {
			Ok(res) => res.to_response(),
			Err(res) => res,
		}
	}
}

#[cfg(feature = "json")]
impl ResponseLike for serde_json::Value {
	#[inline]
	fn to_response(self) -> Response {
		crate::response!(ok, self.to_string())
	}
}
