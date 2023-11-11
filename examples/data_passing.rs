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

		// Access the data
		// Even though data.hello implements ResponseLike, we use
		// response!() to avoid borrow checker issues
		response!(ok, data.hello)
	})
}
