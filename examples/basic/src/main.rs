use snowboard::{response, Method, Server};

fn main() {
	let data = "Hello, world!";

	Server::new("localhost:8080").run(move |mut req| {
		if req.method != Method::GET {
			return response!(method_not_allowed);
		}

		req.set_header("X-Server", "Snowboard");

		println!("{:?}", &req);

		let mut res = response!(ok, data);
		res.set_header("X-Hello", "World".into());
		res
	});
}
