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
    Server::new("localhost:8080".into())
        .on_request(|request| {
            println!("{:?}", request);

            response!(ok, "Hello, world!")
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
    Server::new("localhost:8080")
        .add_middleware(|mut request| {
            if request.method != Method::GET {
                let res = response!(method_not_allowed, "Use GET!");
                return (request, Some(res));
            }

            request.set_header("X-Server", "Snowboard");

            (request, None)
        })
        .on_request(|request| {
            println!("{:?}", request);
            assert_eq!(request.method, Method::GET);
            assert_eq!(request.get_header("X-Server"), Some("Snowboard"));

            response!(ok, "Hello, world!")
        })
        .run();
}
```

More info can be found at [`Server::add_middleware`](./src/lib.rs) in `lib.rs`.

## **Why should I use this?**

Snowboard is designed and created for people who like coding their own things from little to nothing, like me.
This library does not implement what most server libraries have,
but rather offers a plain request-response system that can be heavily configured or changed based on user preference.
It comes with the essential tools for writing whatever you want to.

## **Contributing**

Check [CONTRIBUTING.md](CONTRIBUTING.md) for a simple guide on how to help the project.

## **License**

This code is under the MIT license that can be found at [LICENSE](./LICENSE)
