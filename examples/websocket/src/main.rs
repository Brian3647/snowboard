use std::{collections::HashMap, net::TcpStream};

use snowboard::{response, Request};

use base64::engine::general_purpose::STANDARD as BASE64ENGINE;
use base64::Engine;

use sha1::{Digest, Sha1};

fn build_handshake<'a>(sec_key: String) -> HashMap<&'a str, String> {
	let mut headers = HashMap::new();

	headers.insert("Connection", "Upgrade".into());
	headers.insert("Upgrade", "websocket".into());

	let mut sha1 = Sha1::new();
	sha1.update(sec_key.as_bytes());
	sha1.update("258EAFA5-E914-47DA-95CA-C5AB0DC85B11".as_bytes());
	let accept_value = BASE64ENGINE.encode(sha1.finalize());

	headers.insert("Sec-WebSocket-Accept", accept_value);

	headers
}

fn main() -> anyhow::Result<()> {
	let server = snowboard::Server::new("localhost:8080")?;

	loop {
		let (mut stream, request) = server.try_accept().unwrap();
		if let Ok(req) = &request {
			if req.url.starts_with("/ws") {
				handle(stream, request)?;
				continue;
			} else {
				response!(ok).send_to(&mut stream)?;
			}
		} else {
			handle(stream, request)?;
		}
	}
}

use tungstenite::{Message, WebSocket};

fn handle(mut stream: TcpStream, req: Result<Request, String>) -> anyhow::Result<()> {
	println!("New stream connection: {:?}", stream);

	if let Ok(req) = req {
		if !req.headers.iter().all(|header| {
			header == (&"Upgrade".into(), &"websocket".into())
				|| header == (&"Connection".into(), &"Upgrade".into())
				|| header.0 == "Sec-WebSocket-Key"
				|| header.0 == "Sec-WebSocket-Version"
		}) {
			// A valid HTTP request, but not a WebSocket handshake
			return Ok(());
		}

		let headers: HashMap<&str, String> =
			build_handshake(req.get_header_or("Sec-WebSocket-Key", "").into());

		response!(switching_protocols, "", headers).send_to(&mut stream)?;
	};

	// Why not simply using `WebSocket::connect()`?:
	// Well, snowboard does read the stream buffer to parse the request,
	// which means tungstenite would not be able to read the handshake.
	// So we have to create the WebSocket manually.

	let mut websocket =
		WebSocket::from_raw_socket(stream, tungstenite::protocol::Role::Server, None);

	while let Ok(msg) = websocket.read() {
		match msg {
			Message::Text(text) => {
				println!("Received: {}", text);
				websocket.write(Message::Text(text))?;
				websocket.flush()?;
			}
			Message::Close(_) => {
				websocket.close(None)?;
				break;
			}
			Message::Ping(data) => websocket.write(Message::Pong(data))?,
			_ => {}
		}
	}

	Ok(())
}
