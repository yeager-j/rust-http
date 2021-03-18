mod http;

use http::{Server};

fn main() {
    let server = Server::new("127.0.0.1", 8000);
    server.run();
}
