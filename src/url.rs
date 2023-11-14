use std::{collections::HashMap, fmt::Display};

/// A parsed URL.
#[cfg_attr(feature = "json", derive(serde::Serialize))]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Url<'a> {
	/// Original path, divided by `/`
	pub path: Vec<&'a str>,
	/// Search parameters, specified using `?key=value` in the URL.
	pub search_params: HashMap<&'a str, &'a str>,
}

impl<'a> Url<'a> {
	/// Creates directly a URL.
	/// Use `Url::from` to parse a string.
	pub fn new(path: Vec<&'a str>, search_params: HashMap<&'a str, &'a str>) -> Self {
		Self {
			path,
			search_params,
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
		let (path_part, query_part) = value.split_once('?').unwrap_or((value, ""));
		let path: Vec<&'a str> = path_part.split('/').filter(|x| !x.is_empty()).collect();

		let mut search_params = HashMap::new();

		if !query_part.is_empty() {
			for s in query_part.split('&') {
				let (key, value) = s.split_once('=').unwrap_or((s, ""));
				search_params.insert(key, value);
			}
		}

		Self::new(path, search_params)
	}
}

use std::fmt;

impl Display for Url<'_> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let path_str = self.path.join("/");
		let params = self
			.search_params
			.iter()
			.map(|(key, value)| format!("{}={}", key, value))
			.collect::<Vec<String>>()
			.join("&");

		write!(f, "{}?{}", path_str, params)
	}
}
