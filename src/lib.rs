mod request;
mod response;
mod util;

pub use request::Request;
pub use response::Response;
pub use util::*;

use std::{
    io::Read,
    net::{SocketAddr, TcpListener, TcpStream},
    thread,
};

/// The size of the buffer used to read incoming requests.
/// It's set to 8KB (8192 bytes).
pub const BUFFER_SIZE: usize = 8192;

/// A type alias for any handler function.
pub type Handler = fn(request: Request) -> Response;

/// Middleware function. Returns a tuple of the modified request and an optional response.
/// If the response is not `None`, the request will be ignored and the response will be sent.
pub type Middleware = fn(request: Request) -> (Request, Option<Response>);

/// Simple server struct
pub struct Server {
    addr: String,
    on_request: Option<Handler>,
    middleware: Vec<Middleware>,
}

impl Server {
    /// Create a new server instance.
    pub fn new(addr: String) -> Self {
        Self {
            addr,
            on_request: None,
            middleware: vec![],
        }
    }

    pub fn on_request(&mut self, handler: Handler) -> &mut Self {
        self.on_request = Some(handler);
        self
    }

    pub fn add_middleware(&mut self, handler: Middleware) -> &mut Self {
        self.middleware.push(handler);
        self
    }

    /// Start the server.
    pub fn run(&self) -> ! {
        println!("Listening on {}", self.addr);

        let listener = TcpListener::bind(&self.addr).unwrap();

        loop {
            match listener.accept() {
                Ok(req) => self.spawn_handler(req),
                Err(e) => println!("Failed to establish a connection: {}", e),
            }
        }
    }

    fn spawn_handler(&self, listener: (TcpStream, SocketAddr)) {
        let middleware = self.middleware.clone();
        let handler = self.on_request;

        thread::spawn(move || handle_request(listener, handler, middleware));
    }
}

fn handle_request(
    listener: (TcpStream, SocketAddr),
    handler: Option<Handler>,
    middleware: Vec<Middleware>,
) {
    let (mut stream, ip) = listener;

    let mut buffer = [0; BUFFER_SIZE];
    let read_result = stream.read(&mut buffer);

    if read_result.is_err() {
        println!(
            "Failed to read from connection: {}",
            read_result.err().unwrap()
        );

        return;
    }

    let payload_size = read_result.unwrap();

    if payload_size > BUFFER_SIZE {
        let res = Response::payload_too_large(None, None);
        res.send(&mut stream);
        return;
    }

    let text = String::from_utf8_lossy(&buffer)
        .replace("\0", "")
        .to_string();

    let mut request = Request::new(text, ip);

    for middlewarefn in middleware {
        let (req, res) = middlewarefn(request.clone());

        if let Some(response) = res {
            response.send(&mut stream);
            return;
        }

        request = req;
    }

    let res = match handler {
        Some(handler) => handler(request),
        None => Response::service_unavailable(None, None),
    };

    res.send(&mut stream);
}
