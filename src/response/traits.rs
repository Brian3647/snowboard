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
		let len = self.len();
		crate::response!(ok, self, crate::headers! { "Content-Length" => len })
	}
}

impl ResponseLike for String {
	#[inline]
	fn to_response(self) -> Response {
		let len = self.len();
		crate::response!(
			ok,
			self,
			crate::headers! { "Content-Length" => len, "Content-Type" => "text/plain" }
		)
	}
}

impl ResponseLike for Vec<u8> {
	#[inline]
	fn to_response(self) -> Response {
		let len = self.len();
		crate::response!(
			ok,
			self,
			crate::headers! { "Content-Length" => len, "Content-Type" => "application/octet-stream" }
		)
	}
}

// Particuraly useful for `?` operators when using outside functions.
impl<T, E> ResponseLike for Result<T, E>
where
	T: ResponseLike,
	E: ResponseLike,
{
	fn to_response(self) -> Response {
		match self {
			Ok(res) => res.to_response(),
			Err(res) => res.to_response(),
		}
	}
}

#[cfg(feature = "json")]
impl ResponseLike for serde_json::Value {
	#[inline]
	fn to_response(self) -> Response {
		let bytes = serde_json::to_vec(&self).unwrap_or(self.to_string().into_bytes());
		let len = bytes.len();

		crate::response!(
			ok,
			bytes,
			crate::headers! {
				"Content-Type" => "application/json",
				"Content-Length" => len
			}
		)
	}
}
