use snowboard::{response, Request, Response, Server};

#[derive(Clone)]
struct ServerData {
    // ...
}

fn router(req: Request, _: &ServerData) -> Response<'static> {
    // /{x}
    match req.url().safe_at(0) {
        Some("ping") => response!(ok, "Pong!"),
        Some("api") => response!(not_implemented, '👀'),
        None => response!(ok, "Hello, world!"),
        _ => response!(not_found, "Route not found"),
    }
}

fn main() {
    Server::new("localhost:8080", ServerData {})
        .on_request(router)
        .run();
}
