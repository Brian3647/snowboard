use snowboard::{response, Request, Response, Server};

fn router(req: Request) -> Response {
	// /{x}
	match req.parse_url().at(0) {
		Some("ping") => response!(ok, "Pong!"),
		Some("api") => response!(not_implemented, '👀'),
		None => response!(ok, "Hello, world!"),
		_ => response!(not_found, "Route not found"),
	}
}

fn main() {
	Server::new("localhost:8080").run(router);
}
