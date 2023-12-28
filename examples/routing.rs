use snowboard::{response, Request, ResponseLike, Result, Server};
use std::net::SocketAddr;

fn router(req: Request) -> impl ResponseLike {
	// /{x}
	match req.parse_url().at(0) {
		Some("ping") => response!(ok, "Pong!"),
		Some("api") => response!(not_implemented, "👀"),
		None => response!(ok, "Hello, world!"),
		_ => response!(not_found, "Route not found"),
	}
}

fn main() -> Result {
	Server::new(SocketAddr::from(([0, 0, 0, 0], 3000))).run(router)
}
