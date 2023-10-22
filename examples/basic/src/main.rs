use snowboard::{response, Method, Server};

fn main() {
    Server::new("localhost:8080")
        .add_middleware(|mut request| {
            if request.method != Method::GET {
                let res = response!(method_not_allowed, "Use GET!");
                return (request, Some(res));
            }

            request.set_header("X-Server", "Snowboard");

            (request, None)
        })
        .on_request(|request| {
            println!("{:?}", request);
            assert_eq!(request.method, Method::GET);
            assert_eq!(request.get_header("X-Server"), Some("Snowboard"));

            response!(ok, "Hello, world!")
        })
        .run();
}
