use crate::response::Response;
use crate::{request::Request, Listener};

use std::thread;

/// The size of the buffer used to read incoming requests.
/// It's set to 8KiB by default.
pub const DEFAULT_BUFFER_SIZE: usize = 1024 * 8;

/// A type alias for any handler function.
pub type Handler<T> = fn(request: Request, data: &T) -> Response<'static>;

/// Middleware function. Returns a tuple of the modified request and an optional response.
/// If the response is not `None`, the request will be ignored, and the response will be sent.
pub type Middleware<T> = fn(request: Request, data: &T) -> (Request, Option<Response<'static>>);

/// Simple server struct
pub struct Server<T: Clone + Send + 'static> {
    on_request: Option<Handler<T>>,
    on_load: Option<fn(&T) -> ()>,
    middleware: Vec<Middleware<T>>,
    listener: Listener,
    data: T,
    addr: String,
}

impl<T: Clone + Send + 'static> Server<T> {
    /// Create a new server instance.
    /// The server will not start until the `run` method is called.
    /// The `addr` parameter is a string in the format of `host:port`.
    ///
    /// # Example
    /// See the `basic` example in `examples/basic`.
    pub fn new(addr: impl Into<String>, data: T) -> Self {
        let addr = addr.into();

        Self {
            listener: Listener::new(&addr),
            on_request: None,
            middleware: vec![],
            data,
            on_load: None,
            addr,
        }
    }

    /// Sest the buffer size.
    /// The buffer size is used to read incoming requests.
    /// The default buffer size is 8KB (8192 bytes).
    pub fn set_buffer_size(&mut self, new_size: usize) -> &mut Self {
        self.listener.set_buffer_size(new_size);
        self
    }

    /// Set the handler function.
    /// This function will be called when a request is received.
    /// If no handler is set, the server will respond with a 503 Service Unavailable.
    ///
    /// # Example
    /// ```
    /// use snowboard::Server;
    ///
    /// let server = Server::new("localhost:8080", "")
    ///     .on_request(|request, _| {
    ///         println!("{:?}", request);
    ///
    ///         snowboard::response!(ok, "Hello, world!")
    ///     });
    ///
    /// // server.run();
    /// ```
    pub fn on_request(&mut self, handler: Handler<T>) -> &mut Self {
        self.on_request = Some(handler);
        self
    }

    /// Set the on load function.
    /// This function will be called when the server starts.
    pub fn on_load(&mut self, run: fn(&T) -> ()) -> &mut Self {
        self.on_load = Some(run);
        self
    }

    /// Add a middleware function.
    /// Middleware functions are called before the handler function.
    /// They can be used to modify the request or to return a response.
    /// If a response is returned, the handler function will not be called.
    /// If multiple middleware functions are added, they will be called in the order they were added.
    ///
    /// # Example
    /// ```
    /// use snowboard::Server;
    ///
    /// let mut server = Server::new("localhost:8080", "");
    ///
    /// server.add_middleware(|mut request, _| {
    ///    request.set_header("X-Server", "Snowboard");
    ///
    ///    // Request, Response
    ///    (request, None)
    /// });
    /// ```
    ///
    /// # Example 2
    /// ```
    /// use snowboard::Server;
    ///
    /// let mut server = Server::new("localhost:8080", "");
    ///
    /// server.add_middleware(|request, _| {
    ///    // Request, Response
    ///   (request, Some(snowboard::response!(ok, "Hello, world!")))
    /// });
    /// ```
    pub fn add_middleware(&mut self, handler: Middleware<T>) -> &mut Self {
        self.middleware.push(handler);
        self
    }

    /// Start the server.
    pub fn run(&self) -> ! {
        if let Some(on_load) = self.on_load {
            on_load(&self.data);
        } else {
            println!("[snowboard] Server running @ http://{}", self.addr);
        }

        while let Ok((mut stream, mut request)) = self.listener.try_accept() {
            let middleware = self.middleware.clone();
            let handler = self.on_request;
            let data = self.data.clone();

            thread::spawn(move || {
                for middlewarefn in &middleware {
                    let (req, res) = middlewarefn(request.clone(), &data);

                    if let Some(response) = res {
                        response.send_to(&mut stream);
                        return;
                    }

                    request = req;
                }

                let res = match handler {
                    Some(handler) => handler(request, &data),
                    None => Response::service_unavailable(None, None, None),
                };

                res.send_to(&mut stream);
            });
        }

        unreachable!()
    }
}
