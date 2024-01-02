use snowboard::{response, Request, ResponseLike, Server};
use std::net::SocketAddr;

async fn router(req: Request) -> impl ResponseLike {
	// /{x}
	match req.parse_url().at(0) {
		Some("ping") => response!(ok, "Pong!"),
		Some("api") => response!(not_implemented, "👀"),
		None => response!(ok, "Hello, world!"),
		_ => response!(not_found, "Route not found"),
	}
}

fn main() {
	Server::new(SocketAddr::from(([0, 0, 0, 0], 3000))).run(router)
}
