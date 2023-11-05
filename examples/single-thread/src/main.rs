use snowboard::{response, Result, Server};

fn main() -> Result {
	let server = Server::new("localhost:8080")?;

	for (mut stream, request) in server {
		println!("{:?}", request);

		response!(ok, "Hello, world!").send_to(&mut stream).unwrap();
	}

	unreachable!()
}
