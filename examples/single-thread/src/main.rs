use snowboard::{response, Server};

fn main() {
    let server = Server::new("localhost:8080");

    for (mut stream, request) in server {
        println!("{:?}", request);

        let response = response!(ok, "Hello, world!");

        response.send_to(&mut stream);
    }
}
