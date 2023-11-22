use snowboard::{headers, response, Method, Server};

fn main() -> snowboard::Result {
	let data = "Hello, world!";

	let server = Server::new("localhost:8080")?;

	println!("Listening on {}", server.pretty_addr()?);

	server.run(move |mut req| {
		if req.method == Method::DELETE {
			return response!(method_not_allowed, "Caught you trying to delete!");
		}

		req.set_header("X-Server", "Snowboard");

		println!("{req:#?}");

		response!(ok, data, headers! { "X-Hello" => "World!" })
	})
}
