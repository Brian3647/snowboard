use snowboard::Server;
use snowboard::WebSocket;
use std::net::SocketAddr;

fn handle_ws(mut ws: WebSocket) {
	while let Ok(msg) = ws.read() {
		let _ = ws.send(msg);
	}
}

fn main() -> snowboard::Result {
	Server::new(SocketAddr::from(([0, 0, 0, 0], 3000)))
		.on_websocket("/ws", handle_ws)
		.run(|_| "Try `/ws`!")
}
