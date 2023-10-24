use snowboard::Server;

fn main() {
    Server::new("localhost:8080", "")
        .on_request(|_, _| snowboard::response!(ok, "Pong!"))
        .run();
}
