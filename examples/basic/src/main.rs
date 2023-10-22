use snowboard::{response, Method, Server};

fn main() {
    let data = "Hello, world!";

    Server::new("localhost:8080", data)
        .add_middleware(|mut request, _| {
            if request.method != Method::GET {
                let res = response!(method_not_allowed, "Use GET!");
                return (request, Some(res));
            }

            request.set_header("X-Server", "Snowboard");

            (request, None)
        })
        .on_request(|request, msg| {
            println!("{:?}", request);
            assert_eq!(request.method, Method::GET);
            assert_eq!(request.get_header("X-Server"), Some("Snowboard"));

            response!(ok, msg)
        })
        .run();
}
