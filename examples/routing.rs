use snowboard::{response, Request, ResponseLike, Server};

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
	Server::from_defaults("localhost:3000")
		.expect("Failed to get addr")
		.run(router)
}
