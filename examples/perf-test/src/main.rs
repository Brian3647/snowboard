use snowboard::{response, Server};

fn main() {
	let hello_bytes = &[b'h', b'e', b'l', b'l', b'o'];
	Server::new("localhost:8080").run(move |_| {
		let mut res = response!(ok);
		res.set_bytes(hello_bytes);
		res
	});
}
