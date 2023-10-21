use std::collections::HashMap;
use std::net::SocketAddr;

use crate::Method;

/// A server request.
/// Parses the raw request string into a more usable format.
#[derive(Debug, Clone)]
pub struct Request {
    pub ip: SocketAddr,
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
        let url = parts.next().unwrap().to_string();

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

    pub fn get_header(&self, key: &str) -> Option<&String> {
        self.headers.get(key)
    }

    pub fn get_header_or(&self, key: &str, default: &str) -> String {
        self.get_header(key)
            .unwrap_or(&default.to_string())
            .to_string()
    }
}
