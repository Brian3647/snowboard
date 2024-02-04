use snowboard::{headers, response, Method, Server};

#[tokio::main]
async fn main() -> snowboard::Result {
	let data = "Hello, world!";

	let server = Server::from_defaults("localhost:3000")?;

	println!("Listening on {}", server.pretty_addr());

	server.run(move |mut req| async move {
		if req.method == Method::DELETE {
			return response!(method_not_allowed, "Caught you trying to delete!");
		}

		req.set_header("X-Server", "Snowboard");

		println!("{req:#?}");

		response!(ok, data, headers! { "X-Hello" => "World!" })
	})
}
