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
use snowboard::{response, Server, Result};

fn main() -> Result {
    let hello = "Hello, world!";

    Server::new("localhost:8080")?.run(move |request| {
        println!("{:?}", request);

        response!(ok, hello)
    });
}
```

And that's it! You got yourself a working server on :8080. Examples can be found [here](./examples/).

## **Async routes**

You can use the `async` feature to change `Server::run(..)` so that it supports asynchronous handlers:

```toml
# Cargo.toml

[dependencies]
snowboard = { version = "*", features = ["async"] }
```

```rust
// src/main.rs

use snowboard::async_std::task;
use snowboard::{response, Request, Response, Server, Result};
use std::time::Duration;

async fn index(req: Request) -> Response {
    println!("{:?}", req);
    // Wait 1 second before sending the response
    task::sleep(Duration::from_secs(1)).await;
    response!(ok, "Async works!")
}

fn main() -> Result {
    Server::new("localhost:8080")?.run(index);
}
```

## **TLS**

Use the `tls` feature (which will also install `native-tls`) to change the `Server` struct, adding support for TLS:

```rust
use anyhow::Result;
use snowboard::{
    native_tls::{Identity, TlsAcceptor},
    response, Server,
};

use std::fs;

fn main() -> Result<()> {
    let der = fs::read("identity.pfx")?;
    let password = ..;
    let tls_acceptor = TlsAcceptor::new(Identity::from_pkcs12(&der, password)?)?;

    Server::new("localhost:3000", tls_acceptor)?
        .run(|request| response!(ok, format!("{:?}", request)))
}
```

You can confirm it works by running `curl -k localhost:3000` _(the -k is needed to allow self-signed certificates)_
More info can be found [here](./examples/tls/).

## **Websockets**

Even though websocket isn't supported by default, you can install `tungstenite` and use it without much effort.
Check [`examples/websocket`](./examples/websocket/src/main.rs) for an example.

## **Routing**

Routing can be handled easily using the `Url` struct as seen in [`examples/routing`](./examples/routing/src/main.rs).

## **Why should I use this?**

Snowboard is designed and created for people who like coding their own things from little to nothing, like me.
This library does not implement what most server libraries have, like an advanced routing system,
but rather offers a set of essential tools to create a powerful web server.

## **Contributing**

Check [CONTRIBUTING.md](CONTRIBUTING.md) for a simple guide on how to help the project.

## **License**

This code is under the MIT license that can be found at [LICENSE](./LICENSE)
