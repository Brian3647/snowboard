use snowboard::{response, Method, Server};

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
            assert_eq!(request.method, Method::GET);
            assert_eq!(request.get_header("X-Server"), Some("Snowboard"));

            // Access the data
            response!(ok, my_data.hello)
        })
        .run();
}
