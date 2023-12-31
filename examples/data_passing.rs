use snowboard::{response, Result, Server};

struct ServerData {
	hello: String,
}

fn main() -> Result {
	let data = ServerData {
		hello: "hi!".into(),
	};

	Server::new("localhost:8080")?.run(move |request| {
		println!("{:#?}", request);

		response!(ok, data.hello.clone())
	})
}
