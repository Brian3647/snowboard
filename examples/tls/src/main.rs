use anyhow::Result;
use snowboard::{Identity, TlsAcceptor};

use snowboard::Server;
use snowboard::WebSocket;

use std::fs;
use std::net::SocketAddr;

fn handle_ws(mut ws: WebSocket) {
	while let Ok(msg) = ws.read() {
		let _ = ws.send(msg);
	}
}

fn main() -> Result<()> {
	let der = fs::read("identity.pfx")?;
	let password = "1234";
	let tls_acceptor = TlsAcceptor::new(Identity::from_pkcs12(&der, password)?)?;

	Server::from_defaults(SocketAddr::from(([0, 0, 0, 0], 3000)))?
		.with_tls(tls_acceptor)
		.run(|request| async move { format!("{request:#?}") })
}
