use std::{collections::HashMap, fmt::Display};

/// A parsed URL.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Url<'a> {
	/// Original path, divided by `/`
	pub path: Vec<&'a str>,
	/// Search parameters, specified using `?key=value` in the URL.
	pub search_params: HashMap<&'a str, &'a str>,
	/// Fragment, specified using `#fragment` in the URL.
	pub fragment: Option<&'a str>,
	/// Raw copy of the original URL.
	pub raw: String,
}

impl<'a> Url<'a> {
	/// Creates directly a URL, and generates `raw`.
	/// Use `Url::from` to parse a string.
	pub fn new(
		path: Vec<&'a str>,
		search_params: HashMap<&'a str, &'a str>,
		fragment: Option<&'a str>,
	) -> Self {
		let mut raw = String::new();

		for (i, p) in path.iter().enumerate() {
			if i > 0 {
				raw.push('/');
			}

			raw.push_str(p);
		}

		if !search_params.is_empty() {
			raw.push('?');

			for (i, (k, v)) in search_params.iter().enumerate() {
				if i > 0 {
					raw.push('&');
				}

				raw.push_str(k);
				raw.push('=');
				raw.push_str(v);
			}
		}

		if let Some(f) = fragment {
			raw.push('#');
			raw.push_str(f);
		}

		Self {
			path,
			search_params,
			fragment,
			raw,
		}
	}

	/// Returns the `i` element of the path.
	/// If the element does not exist, returns `None`.
	pub fn at(&self, i: usize) -> Option<&'a str> {
		self.path.get(i).copied()
	}

	/// Gets a copy of a search parameter.
	pub fn search_param(&self, key: &'a str) -> Option<&'a str> {
		self.search_params.get(key).copied()
	}
}

impl<'a> From<&'a str> for Url<'a> {
	fn from(value: &'a str) -> Self {
		let parts: Vec<&'a str> = value.split('?').collect();
		let path: Vec<&'a str> = parts[0].split('/').filter(|x| !x.is_empty()).collect();
		let mut search_params = HashMap::new();
		let mut fragment = None;

		if parts.len() > 1 {
			let query: Vec<&'a str> = parts[1].split('#').collect();
			for s in query[0].split('&') {
				let pair: Vec<&'a str> = s.split('=').collect();
				search_params.insert(pair[0], *pair.get(1).unwrap_or(&""));
			}

			if query.len() > 1 {
				fragment = Some(query[1]);
			}
		}

		Self::new(path, search_params, fragment)
	}
}

use std::fmt;

impl Display for Url<'_> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}", self.raw)
	}
}
