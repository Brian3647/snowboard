use crate::request::Request;
use crate::response::Response;

#[cfg(feature = "async")]
use std::future::Future;

/// The size of the buffer used to read incoming requests.
/// It's set to 8KiB by default.
pub const DEFAULT_BUFFER_SIZE: usize = 1024 * 8;

use std::{
    io::{Error, ErrorKind, Read, Result},
    net::{TcpListener, TcpStream},
};

#[cfg(not(feature = "async"))]
use std::thread;

/// Single threaded listener made for simpler servers.
#[derive(Debug)]
pub struct Server {
    tcp_listener: TcpListener,
    buffer_size: usize,
}

/// Simple rust TCP HTTP server.
impl Server {
    /// Create a new server instance.
    /// The server will listen on the given address.
    /// The address must be in the format of `ip:port`.
    pub fn new(addr: impl Into<String>) -> Self {
        let addr = addr.into();

        Self {
            tcp_listener: TcpListener::bind(addr).unwrap(),
            buffer_size: DEFAULT_BUFFER_SIZE,
        }
    }

    /// Set the buffer size used to read incoming requests.
    /// The default buffer size is 8KiB.
    /// The buffer size must be greater than 0.
    pub fn set_buffer_size(&mut self, size: usize) {
        assert!(size > 0, "Buffer size must be greater than 0");

        self.buffer_size = size;
    }

    /// Try to accept a new incoming request.
    /// Returns an error if the request could not be read.
    /// The request will be read into a buffer and parsed into a `Request` instance.
    /// The buffer size can be changed with `Server::set_buffer_size()`.
    /// The default buffer size is 8KiB.
    /// The buffer size must be greater than 0.
    ///
    /// # Example
    /// ```no_run
    /// use snowboard::{Request, Response, Server};
    ///
    /// let server = Server::new("localhost:8080");
    ///
    /// loop {
    ///    match server.try_accept() {
    ///       Ok((stream, request)) => {
    ///         // Handle request
    ///      },
    ///      Err(_) => {
    ///        // Handle error
    ///      }
    ///   }
    /// }
    /// ```
    pub fn try_accept(&self) -> Result<(TcpStream, Request)> {
        let stream = self.tcp_listener.accept()?;

        let (mut stream, ip) = stream;

        let mut buffer: Vec<u8> = vec![0; self.buffer_size];
        let payload_size = stream.read(&mut buffer)?;

        if payload_size > self.buffer_size {
            crate::response!(payload_too_large).send_to(&mut stream)?;
            return Err(Error::new(ErrorKind::InvalidInput, "Payload too large"));
        }

        if payload_size == 0 {
            crate::response!(bad_request).send_to(&mut stream)?;
            return Err(Error::new(ErrorKind::InvalidInput, "Empty request"));
        }

        let text = String::from_utf8_lossy(&buffer).replace('\0', "");

        let req = match Request::new(text, ip) {
            Some(req) => req,
            None => {
                crate::response!(bad_request).send_to(&mut stream)?;
                return Err(Error::new(ErrorKind::InvalidInput, "Invalid request"));
            }
        };

        Ok((stream, req))
    }

    /// Run a multi-thread listener from a handler function.
    /// The handler function will be called when a request is received.
    /// The handler function must return a `Response` instance. Meant for servers
    /// where passing data to the handler is not needed.
    ///
    /// # Example
    /// ```no_run
    /// use snowboard::{response, Server};
    ///
    /// Server::new("localhost:8080").run(|_| response!(ok));
    /// ```
    #[cfg(not(feature = "async"))]
    pub fn run(self, handler: impl Fn(Request) -> Response + Send + 'static + Clone) -> ! {
        for (mut stream, request) in self {
            let handler = handler.clone();

            thread::spawn(move || {
                let response = handler(request);
                if let Err(e) = response.send_to(&mut stream) {
                    eprintln!("Error writing response: {:?}", e);
                };
            });
        }

        unreachable!("Server::run() should never return")
    }

    /// Runs the server asynchronously.
    ///
    /// This function takes a handler function as an argument. The handler function is expected to be a
    /// function that takes a `Request` and returns a `Future` that resolves to a `Response<'static>`.
    ///
    /// The handler function is cloned for each request, and each request is processed in a separate
    /// async task. This means that multiple requests can be processed concurrently.
    ///
    /// This function is only available when the `async` feature is enabled.
    /// # Examples
    ///
    /// # Example
    /// ```no_run
    /// use snowboard::{response, Server};
    ///
    /// Server::new("localhost:8080").run(async |_| response!(ok));
    /// ```
    #[cfg(feature = "async")]
    pub fn run<H, F>(self, handler: H) -> !
    where
        H: Fn(Request) -> F + Clone + Send + 'static + Sync,
        F: Future<Output = Response> + Send + 'static,
    {
        use async_std::task;

        for (mut stream, request) in self {
            let handler = handler.clone();

            task::spawn(async move {
                if let Err(e) = handler(request).await.send_to(&mut stream) {
                    eprintln!("Error writing response: {:?}", e);
                };
            });
        }

        unreachable!("Server::run() should never return")
    }
}

impl Iterator for Server {
    type Item = (TcpStream, Request);

    fn next(&mut self) -> Option<Self::Item> {
        match self.try_accept() {
            Ok(req) => Some(req),
            Err(e) => {
                if e.kind() != ErrorKind::InvalidInput {
                    eprintln!("Error: {:?}", e);
                }

                // Try again
                self.next()
            }
        }
    }
}
