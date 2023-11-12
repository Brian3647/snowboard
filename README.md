# **Snowboard 🏂**

![License](https://img.shields.io/github/license/Brian3647/snowboard)
![GitHub issues](https://img.shields.io/github/issues/Brian3647/snowboard)
![Build status](https://img.shields.io/github/actions/workflow/status/Brian3647/snowboard/rust.yml)
[![DeepSource](https://app.deepsource.com/gh/Brian3647/snowboard.svg/?label=active+issues&show_trend=false)](https://app.deepsource.com/gh/Brian3647/snowboard/)
[![dependency status](https://deps.rs/repo/github/Brian3647/snowboard/status.svg)](https://deps.rs/repo/github/Brian3647/snowboard)

An extremely simple library for fast HTTP & HTTPS servers in Rust

\[[Request a feature/Report a bug](https://github.com/Brian3647/snowboard/issues)\]

## **Quick start**

To get started with Snowboard, simply add it to your `Cargo.toml` file:

```toml
[dependencies]
snowboard = "*"
```

Then, create a new Rust file with the following code:

```rust
use snowboard::{headers, response, Method, Result, Server};

fn main() -> Result {
    let data = "Hello, world!";

    let server = Server::new("localhost:8080")?;

    println!("Listening on {}", server.addr().unwrap());

    server.run(move |mut req| {
        if req.method != Method::GET {
            return response!(method_not_allowed);
        }

        req.set_header("X-Server", "Snowboard");

        println!("{:#?}", &req);

        response!(ok, data, headers! { "X-Hello" => "World!" })
    });
}
```

And that's it! You got yourself a working server on :8080. Examples can be found in the `examples` folder.

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
use snowboard::{Request, ResponseLike, Server, Result};
use std::time::Duration;

async fn index(_: Request) -> impl ResponseLike {
    // Wait 1 second before sending the response
    task::sleep(Duration::from_secs(1)).await;

    "Async works"
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
        .run(|request| format!("{:#?}", request))
}
```

You can confirm it works by running `curl -k localhost:3000` _(the -k is needed to allow self-signed certificates)_

More info can be found in `examples/tls`.

## **Websockets**

Even though websocket isn't supported by default, you can install `tungstenite` and use it without much effort.
Check `examples/websocket` for an example.

## **Routing**

Routing can be handled easily using the `Url` struct as seen in `examples/routing.rs`.

## **Why should I use this?**

Snowboard is designed and created for people who like coding their own things from little to nothing, like me.
This library does not implement what most server libraries have, like an advanced routing system,
but rather offers a set of essential tools to create a powerful web server.

## **Contributing**

Check [CONTRIBUTING.md](CONTRIBUTING.md) for a simple guide on how to help the project.

## **License**

This code is under the MIT license that can be found at [LICENSE](./LICENSE)
