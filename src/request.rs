use std::collections::HashMap;
use std::net::SocketAddr;

use crate::{Method, Url};

/// A server request.
/// Parses the raw request string into a more usable format.
#[derive(Debug, Clone)]
pub struct Request {
    pub ip: SocketAddr,
    /// Raw URL string.
    /// Use `Request::url()` to get a parsed version of the URL
    pub url: String,
    pub method: Method,
    pub body: String,
    pub headers: HashMap<String, String>,
}

impl Request {
    pub fn new(text: String, ip: SocketAddr) -> Self {
        let mut lines = text.lines();

        let first_line = lines.next().unwrap();
        let mut parts = first_line.split_whitespace();

        let method = Method::from(parts.next().unwrap().to_string());
        let url = parts.next().unwrap().into();

        let mut headers = HashMap::new();
        let mut in_body = false;
        let mut body = String::new();

        for line in lines {
            if line.is_empty() {
                in_body = true;
                continue;
            } else if in_body {
                body.push_str(line);
                continue;
            }

            let mut parts = line.splitn(2, ':');
            let key = parts.next().unwrap().into();
            let value = parts.next().unwrap().trim().into();

            headers.insert(key, value);
        }

        Self {
            ip,
            url,
            method,
            body,
            headers,
        }
    }

    pub fn get_header(&self, key: &str) -> Option<&str> {
        self.headers.get(key).map(|s| s.as_str())
    }

    pub fn get_header_or(&self, key: &str, default: &'static str) -> &str {
        self.get_header(key).unwrap_or(default)
    }

    pub fn set_header<T, K>(&mut self, key: T, value: K)
    where
        T: Into<String>,
        K: Into<String>,
    {
        self.headers.insert(key.into(), value.into());
    }

    /// Get a parsed version of the URL
    pub fn parse_url(&self) -> Url {
        self.url.as_str().into()
    }
}
