use snowboard::async_std::task;
use snowboard::{response, Request, Response, Result, Server};
use std::time::Duration;

async fn index(req: Request) -> Response {
	println!("{:#?}", req);
	task::sleep(Duration::from_secs(1)).await;
	response!(ok, "Async works!")
}

fn main() -> Result {
	Server::new("localhost:8080")?.run(index);
}
