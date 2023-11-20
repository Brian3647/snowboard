use async_std::task;
use snowboard::{Request, ResponseLike, Result, Server};
use std::time::Duration;

async fn index(_: Request) -> impl ResponseLike {
	task::sleep(Duration::from_secs(1)).await;

	"Async works!"
}

async fn ws_handler(mut ws: snowboard::WebSocket<'_>) {
	while let Ok(msg) = ws.read() {
		let _ = ws.send(msg);
	}
}

fn main() -> Result {
	Server::new("localhost:8080")?
		.on_websocket("/ws", |ws| async_std::task::block_on(ws_handler(ws)))
		.run_async(index);
}
