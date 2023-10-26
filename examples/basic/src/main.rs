use std::collections::HashMap;

use snowboard::{response, Method, Server};

fn main() {
    let data = "Hello, world!";

    Server::new("localhost:8080").run(move |mut req| {
        if req.method != Method::GET {
            return response!(method_not_allowed);
        }

        req.set_header("X-Server", "Snowboard");

        println!("{:?}", req);

        let mut headers = HashMap::new();
        headers.insert("X-Hello", "World");

        response!(ok, data, headers)
    });
}
