use anyhow::Result;
use snowboard::{Identity, TlsAcceptor, TlsStream};
use std::net::TcpStream;

use snowboard::Server;
use snowboard::WebSocket;

use std::fs;

fn handle_ws(mut ws: WebSocket<&mut TlsStream<TcpStream>>) {
	while let Ok(msg) = ws.read() {
		ws.send(msg).unwrap();
	}
}

fn main() -> Result<()> {
	let der = fs::read("identity.pfx")?;
	let password = "1234";
	let tls_acceptor = TlsAcceptor::new(Identity::from_pkcs12(&der, password)?)?;

	Server::new("localhost:3000", tls_acceptor)?
		.on_websocket("/ws", handle_ws)
		.run(|request| format!("{request:#?}"))
}
