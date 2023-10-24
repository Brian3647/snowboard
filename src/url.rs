#[derive(Debug, Clone)]
pub struct Url<'a> {
    pub path: Vec<&'a str>,
    pub search_params: Vec<(&'a str, &'a str)>,
}

impl<'a> Url<'a> {
    pub fn new(path: Vec<&'a str>, search_params: Vec<(&'a str, &'a str)>) -> Self {
        Self {
            path,
            search_params,
        }
    }

    pub fn at(&self, i: usize) -> &'a str {
        self.path[i]
    }

    /// Returns the `i` element of the path.
    /// If the element does not exist, returns `None`.
    pub fn safe_at(&self, i: usize) -> Option<&'a str> {
        self.path.get(i).copied()
    }

    pub fn search_param(&self, key: &'a str) -> Option<&'a str> {
        for (k, v) in &self.search_params {
            if k == &key {
                return Some(v);
            }
        }

        None
    }
}

impl<'a> From<&'a str> for Url<'a> {
    fn from(value: &'a str) -> Self {
        let mut parts = value.split('?');
        let mut path = parts
            .next()
            .unwrap_or_default()
            .split('/')
            .skip(1)
            .collect::<Vec<&str>>();

        // When requesting /, path becomes vec![""], and this filters that issue.
        if path == vec![""] {
            path = vec![];
        }

        if let Some(sp) = parts.next() {
            let search_params = sp
                .split('&')
                .map(|param| {
                    let mut parts = param.split('=');
                    let key = parts.next().unwrap();
                    let value = parts.next().unwrap_or_default();

                    (key, value)
                })
                .collect::<Vec<(&str, &str)>>();

            return Self::new(path, search_params);
        }

        Self::new(path, vec![])
    }
}
