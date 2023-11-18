#[macro_export]
macro_rules! map_into {
	($($name:expr => $val:expr $(,)?)*) => {
		{
			let mut map = HashMap::new();
			$(map.insert($name.into(), $val.into());)*
			map
		}
	};
}

use std::collections::HashMap;

use snowboard::{Method, Request, Url};

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

#[test]
fn parse_url() {
	let complex = "/path/to/something?param1=value1&param2=value2&param3=value3&s=&";
	let simple = "/a/b?c=d";
	let base = "/";
	let weird = "/?&=";
	let no_query = "/a/b/c";

	assert_eq!(
		Url::from(complex),
		Url {
			path: vec!["path", "to", "something"],
			search_params: map_into! {
				"param1" => "value1",
				"param2" => "value2",
				"param3" => "value3",
				"s" => "",
			}
		}
	);

	assert_eq!(
		Url::from(simple),
		Url {
			path: vec!["a", "b"],
			search_params: map_into! {
				"c" => "d",
			}
		}
	);

	assert_eq!(
		Url::from(base),
		Url {
			path: vec![],
			search_params: HashMap::new(),
		}
	);

	assert_eq!(
		Url::from(weird),
		Url {
			path: vec![],
			search_params: HashMap::new()
		}
	);

	assert_eq!(
		Url::from(no_query),
		Url {
			path: vec!["a", "b", "c"],
			search_params: HashMap::new()
		}
	);
}
