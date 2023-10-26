# **Snowboard 🏂**

![License](https://img.shields.io/github/license/Brian3647/snowboard)
![GitHub issues](https://img.shields.io/github/issues/Brian3647/snowboard)
![Build status](https://img.shields.io/github/actions/workflow/status/Brian3647/snowboard/rust.yml)

An extremelly simple library for fast & simple TCP servers in rust

\[[Request a feature/Report a bug](https://github.com/Brian3647/snowboard/issues)\]

## **Quick start**

To get started with Snowboard, simply add it to your `Cargo.toml` file:

```toml
[dependencies]
snowboard = "*"
```

Then, create a new Rust file with the following code:

```rust
use snowboard::{response, Server};

fn main() {
    let data = "Hello, world!";

    Server::new("localhost:8080").run(move |request| {
        println!("{:?}", request);

        response!(ok, data)
    })
}
```

And that's it! You got yourself a working server on :8080. Examples can be found [here](./examples/).

## **Async routes**

You can use the `async` feature to change `Server::run()` so that it supports asynchronous handlers:

```toml
# Cargo.toml

[dependencies]
snowboard = { version = "*", features = ["async"] }
```

```rust
// src/main.rs

use snowboard::async_std::task;
use snowboard::{response, Request, Response, Server};
use std::time::Duration;

async fn index(req: Request) -> Response {
    println!("{:?}", req);
    // Wait 1 second before sending the response
    task::sleep(Duration::from_secs(1)).await;
    response!(ok, "Async works!")
}

fn main() {
    Server::new("localhost:8080").run(index);
}
```

## **Routing**

Routing can be handled easily using the `Url` struct as seen in [`examples/routing`](./examples/routing/).

## **Why should I use this?**

Snowboard is designed and created for people who like coding their own things from little to nothing, like me.
This library does not implement what most server libraries have, like an advanced routing system,
but rather offers a set of essential tools to create a powerful web server.

## **Contributing**

Check [CONTRIBUTING.md](CONTRIBUTING.md) for a simple guide on how to help the project.

## **License**

This code is under the MIT license that can be found at [LICENSE](./LICENSE)
