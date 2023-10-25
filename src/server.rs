use crate::request::Request;
use crate::response::Response;

/// The size of the buffer used to read incoming requests.
/// It's set to 8KiB by default.
pub const DEFAULT_BUFFER_SIZE: usize = 1024 * 8;

use std::{
    io::{Error, ErrorKind, Read, Result},
    net::{TcpListener, TcpStream},
    thread,
};

/// Single threaded listener made for simpler servers.
#[derive(Debug)]
pub struct Server {
    tcp_listener: TcpListener,
    buffer_size: usize,
}

impl Server {
    pub fn new(addr: impl Into<String>) -> Self {
        let addr = addr.into();

        Self {
            tcp_listener: TcpListener::bind(addr).unwrap(),
            buffer_size: DEFAULT_BUFFER_SIZE,
        }
    }

    pub fn set_buffer_size(&mut self, size: usize) {
        assert!(size > 0, "Buffer size must be greater than 0");

        self.buffer_size = size;
    }

    pub fn try_accept(&self) -> Result<(TcpStream, Request)> {
        let stream = self.tcp_listener.accept()?;

        let (mut stream, ip) = stream;

        let mut buffer: Vec<u8> = vec![0; self.buffer_size];
        let payload_size = stream.read(&mut buffer)?;

        if payload_size > self.buffer_size {
            Response::payload_too_large(None, None, None).send_to(&mut stream);
            return Err(Error::new(ErrorKind::InvalidInput, "Payload too large"));
        }

        let text = String::from_utf8_lossy(&buffer).replace('\0', "");

        Ok((stream, Request::new(text, ip)))
    }

    /// Run a multi-thread listener from a handler function.
    /// The handler function will be called when a request is received.
    /// The handler function must return a `Response` instance. Meant for servers
    /// where passing data to the handler is not needed.
    ///
    /// # Example
    /// ```
    /// use snowboard::{response, Listener};
    ///
    /// Listener::run_from_handler("localhost:8080", |_| response!(ok));
    /// ```
    pub fn run(self, handler: impl Fn(Request) -> Response<'static> + Send + 'static + Clone) -> ! {
        for (mut stream, request) in self {
            let handler = handler.clone();

            thread::spawn(move || {
                let response = handler(request);
                response.send_to(&mut stream);
            });
        }

        unreachable!()
    }
}

impl Iterator for Server {
    type Item = (TcpStream, Request);

    fn next(&mut self) -> Option<Self::Item> {
        match self.try_accept() {
            Ok(req) => Some(req),
            Err(_) => None,
        }
    }
}
