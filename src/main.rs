#![allow(dead_code)]
mod http;

use http::Server;

fn main() {
    let server = Server::new("127.0.0.1", 8000);

    match server.run() {
        Ok(_) => {}
        Err(e) => println!("Server encountered an error! {}", e),
    }
}
