use criterion::{criterion_group, criterion_main, Criterion};
use snowboard::Request;

fn parse_request(c: &mut Criterion) {
	let bytes = b"GET /path HTTP/1.1\r\nContent-Length: 10\r\n\r\n0123456789".to_vec();
	let ip = "127.0.0.1:8080".parse().unwrap();

	c.bench_function("parse_request", |b| {
		b.iter(|| {
			Request::new(bytes.clone(), ip);
		})
	});
}

criterion_group!(benches, parse_request);
criterion_main!(benches);
