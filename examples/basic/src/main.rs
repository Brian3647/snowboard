use snowboard::Server;

fn main() {
    Server::new("localhost:8080".into())
        .add_middleware(|mut request| {
            request.set_header("X-Server", "Snowboard");

            (request, None)
        })
        .on_request(|request| {
            println!("{:?}", request);
            assert_eq!(request.get_header("X-Server"), Some(&"Snowboard".into()));

            snowboard::response!(ok, "Hello, world!")
        })
        .run();
}
