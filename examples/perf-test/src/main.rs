use snowboard::{response, Result, Server};

fn main() -> Result {
	let hello_bytes = b"Hello, world!";
	Server::new("localhost:8080")?.run(move |_| {
		let mut res = response!(ok);
		res.set_bytes(hello_bytes);
		res
	});
}
