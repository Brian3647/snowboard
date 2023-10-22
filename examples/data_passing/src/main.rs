use snowboard::{response, Server};

#[derive(Clone)]
struct ServerData {
    hello: String,
}

fn main() {
    let data = ServerData {
        hello: "hi!".into(),
    };

    Server::new("localhost:8080", data)
        .on_request(|request, my_data| {
            println!("{:?}", request);

            // Access the data
            response!(ok, my_data.hello)
        })
        .run();
}
