use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Url<'a> {
	pub path: Vec<&'a str>,
	pub search_params: HashMap<&'a str, &'a str>,
	pub fragment: Option<&'a str>,
}

impl<'a> Url<'a> {
	pub fn new(
		path: Vec<&'a str>,
		search_params: HashMap<&'a str, &'a str>,
		fragment: Option<&'a str>,
	) -> Self {
		Self {
			path,
			search_params,
			fragment,
		}
	}

	/// Returns the `i` element of the path.
	/// If the element does not exist, returns `None`.
	pub fn at(&self, i: usize) -> Option<&'a str> {
		self.path.get(i).copied()
	}

	pub fn search_param(&self, key: &'a str) -> Option<&'a str> {
		self.search_params.get(key).copied()
	}
}

impl<'a> From<&'a str> for Url<'a> {
	fn from(value: &'a str) -> Self {
		let parts: Vec<&'a str> = value.split('?').collect();
		let path: Vec<&'a str> = parts[0].split('/').collect();
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
