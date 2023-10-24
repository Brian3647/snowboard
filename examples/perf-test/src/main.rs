use std::thread;

use snowboard::{response, Listener};
fn main() {
    let server = Listener::new("localhost:8080");

    for (mut stream, _) in server {
        thread::spawn(move || response!(ok).send_to(&mut stream));
    }
}
