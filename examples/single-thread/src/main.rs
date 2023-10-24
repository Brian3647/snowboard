use snowboard::{response, Listener};

fn main() {
    let server = Listener::new("localhost:8080");

    for (mut stream, request) in server {
        println!("{:?}", request);

        let response = response!(ok, "Hello, world!");

        response.send_to(&mut stream);
    }
}
