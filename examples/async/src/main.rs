use snowboard::async_std::task;
use snowboard::{Request, ResponseLike, Result, Server};
use std::time::Duration;

async fn index(req: Request) -> impl ResponseLike {
	println!("{:#?}", req);
	task::sleep(Duration::from_secs(1)).await;
	"Async works!"
}

fn main() -> Result {
	Server::new("localhost:8080")?.run(index);
}
