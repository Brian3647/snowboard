use snowboard::{response, Result, Server};
use std::{net::SocketAddr, sync::Arc};

struct ServerData {
	hello: String,
}

fn main() -> Result {
	let data = Arc::new(ServerData {
		hello: "hi!".into(),
	});

	let data_arc = Arc::clone(&data);

	Server::new(SocketAddr::from(([0, 0, 0, 0], 3000))).run(move |request| {
		let data = Arc::clone(&data_arc);
		async move {
			println!("{:#?}", request);

			response!(ok, data.hello.clone())
		}
	})
}
