# **Snowboard 🏂**

![License](https://img.shields.io/github/license/Brian3647/snowboard)
![GitHub issues](https://img.shields.io/github/issues/Brian3647/snowboard)
![Build status](https://img.shields.io/github/actions/workflow/status/Brian3647/snowboard/rust.yml)

A 0-dependency library for fast & simple TCP servers in rust

\[[Request a feature/Report a bug](https://github.com/Brian3647/snowboard/issues)\]

## **Quick start**

To get started with Snowboard, simply add it to your `Cargo.toml` file:

```toml
[dependencies]
snowboard = "*"
```

Then, create a new Rust file with the following code:

```rust
use snowboard::{Server, response};

fn main() {
    let data = "Hello, world!";

    Server::new("localhost:8080".into(), data)
        .on_request(|request, my_data| {
            println!("{:?}", request);

            response!(ok, my_data)
        })
        .run();
}
```

And that's it! You got yourself a working server on :8080.

## **Features**

### **Middleware**

Adding middleware is extremely easy. You can use the `.add_middleware` function in server to change the request or directly send a response:

```rust
use snowboard::{response, Method, Server};

fn main() {
    let data = "Hello, world!";
    Server::new("localhost:8080", data)
        .add_middleware(|mut request, _| {
            if request.method != Method::GET {
                let res = response!(method_not_allowed, "Use GET!");
                return (request, Some(res));
            }

            request.set_header("X-Server", "Snowboard");

            (request, None)
        })
        .on_request(|request, msg| {
            println!("{:?}", request);
            assert_eq!(request.method, Method::GET);
            assert_eq!(request.get_header("X-Server"), Some("Snowboard"));

            response!(ok, msg)
        })
        .run();
}
```

More info can be found at [`Server::add_middleware`](./src/lib.rs) in `lib.rs`.

### **Passing data**

You can create an original data variable and get its reference on every middleware function/at the handler. You can use anything that implements `Clone` as data:

```rust
use snowboard::{response, Server};

#[derive(Clone)]
struct ServerData {
    hello: String
}

fn main() {
    let data = ServerData {
        hello: "hi!".into()
    };

    Server::new("localhost:8080", data)
        .on_request(|request, my_data| {
            println!("{:?}", request);

            // Access the data
            response!(ok, my_data.hello)
        })
        .run();
}
```

### **Simple listener struct**

If you want to make it even more your own, or use single threads, you can simply use the `Listener` class. This is faster and way better if you do not need data passing. The next code is pretty much what the Server class does, but without data passing, which is sometimes not needed at all:

```rust
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
```

## **Routing**

Routing can be handled easily using the `Url` struct as seen in [`examples/routing`](./examples/routing/).

## **On load function**

_This isn't relevant enough to bloat the readme, but you can find it in lib.ts at [`Server::on_load`]_

## **Why should I use this?**

Snowboard is designed and created for people who like coding their own things from little to nothing, like me.
This library does not implement what most server libraries have, like an advanced routing system,
but rather offers a set of essential tools to create a powerful web server.

## **Examples**

Examples can be found [here](./examples/).

## **Contributing**

Check [CONTRIBUTING.md](CONTRIBUTING.md) for a simple guide on how to help the project.

## **License**

This code is under the MIT license that can be found at [LICENSE](./LICENSE)
