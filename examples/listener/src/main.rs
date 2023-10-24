use std::thread;

use snowboard::{response, Listener, Request, Response};

fn index(request: Request) -> Response<'static> {
    println!("Request: {:?}", request);

    response!(ok, "Hello, world!")
}

fn main() {
    let server = Listener::new("localhost:8080");

    for (mut stream, request) in server {
        thread::spawn(move || index(request).send_to(&mut stream));
    }
}
