use snowboard::{headers, response, Method, Result, Server};

fn main() -> Result {
	let data = "Hello, world!";

	let server = Server::new("localhost:8080")?;

	println!("Listening on {}", server.addr().unwrap());

	server.run(move |mut req| {
		if req.method != Method::GET {
			return response!(method_not_allowed);
		}

		req.set_header("X-Server", "Snowboard");

		println!("{:#?}", &req);

		response!(ok, data, headers! { "X-Hello" => "World!" })
	});
}