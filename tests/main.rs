use snowboard::{Method, Request};

macro_rules! map_into {
	($($name:expr => $val:expr $(,)?)*) => {
		{
			let mut map = ::std::collections::HashMap::new();
			$(map.insert($name.into(), $val.into());)*
			map
		}
	};
}

#[test]
fn parse_request() {
	let request = b"HEAD / HTTP/1.1\r\nHost: localhost:8080\r\nUser-Agent: curl/xx\r\nAccept: */*\r\n\r\nBODY, BODY, BODY";

	let sample_ip = "127.0.0.1:8080".parse().unwrap();

	assert_eq!(
		Request::new(request.to_vec(), sample_ip).unwrap(),
		Request {
			ip: sample_ip,
			url: "/".into(),
			method: Method::HEAD,
			body: "BODY, BODY, BODY".into(),
			headers: map_into! {
				"Host" => "localhost:8080",
				"User-Agent" => "curl/xx",
				"Accept" => "*/*",
			}
		}
	);
}
