use snowboard::Server;

fn main() -> snowboard::Result {
	Server::new("localhost:3000")?.run(|r| {
		serde_json::json!({
			"ip": r.pretty_ip(),
			"url": r.parse_url(),
			"method": r.method,
			"body": r.text(),
			"headers": r.headers,
		})
	})
}
