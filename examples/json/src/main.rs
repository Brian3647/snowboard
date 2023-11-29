use serde_json::Value;
use snowboard::{Response, Server};

#[derive(serde::Deserialize)]
struct Example {
	number: isize,
}

fn main() -> snowboard::Result {
	Server::new("localhost:8080")?.run(|req| -> Result<Value, Response> {
		let example: Example = req.force_json()?;

		Ok(serde_json::json!({
			"number_plus_one": example.number + 1
		}))
	});
}
