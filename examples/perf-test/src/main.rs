use snowboard::{response, Server};

fn main() {
    Server::new("localhost:8080").run(|_| response!(ok));
}
