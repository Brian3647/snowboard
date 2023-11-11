use snowboard::{Result, Server};

fn main() -> Result {
	// Not returning anything (`()`) is the same as Response::default()
	Server::new("localhost:8080")?.run(|_| async {});
}
