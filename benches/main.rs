use criterion::{criterion_group, criterion_main, Criterion};
use snowboard::{Request, Url};

fn parse_request(c: &mut Criterion) {
	let bytes = b"GET /path HTTP/1.1\r\nContent-Length: 10\r\n\r\n0123456789".to_vec();
	let ip = "127.0.0.1:8080".parse().unwrap();

	c.bench_function("parse_request", |b| {
		b.iter(|| {
			Request::new(bytes.clone(), ip);
		})
	});

	let complex_url = "/path/to/something?param1=value1&param2=value2&param3=value3&s=&";
	let simple_url = "/a/b?c=d";
	let base_url = "/";

	c.bench_function("parse_complex_url", |b| b.iter(|| Url::from(complex_url)))
		.bench_function("parse_simple_url", |b| b.iter(|| Url::from(simple_url)))
		.bench_function("parse_base_url", |b| b.iter(|| Url::from(base_url)));
}

criterion_group!(benches, parse_request);
criterion_main!(benches);
