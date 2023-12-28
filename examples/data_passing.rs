use snowboard::{response, Result, Server};
use std::net::SocketAddr;

struct ServerData {
	hello: String,
}

fn main() -> Result {
	let data = ServerData {
		hello: "hi!".into(),
	};

	Server::new(SocketAddr::from(([0, 0, 0, 0], 3000))).run(move |request| {
		println!("{:#?}", request);

		response!(ok, data.hello.clone())
	})
}
