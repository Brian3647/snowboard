use snowboard::Server;

fn main() {
    Server::new("localhost:8080".into())
        .on_request(|request| {
            println!("{:?}", request);

            snowboard::response!(ok, "Hello, world!")
        })
        .run();
}
