use snowboard::{response, Result, Server};

fn main() -> Result {
	Server::new("localhost:8080")?.run(|_| async { response!(ok) });
}
