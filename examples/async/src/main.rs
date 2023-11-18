use async_std::task;
use snowboard::{Request, ResponseLike, Result, Server};
use std::time::Duration;

async fn index(_: Request) -> impl ResponseLike {
	task::sleep(Duration::from_secs(1)).await;

	"Async works!"
}

fn main() -> Result {
	Server::new("localhost:8080")?.run_async(index);
}
