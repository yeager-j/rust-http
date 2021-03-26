#![allow(dead_code)]
mod handler;
mod http;

use crate::handler::WebHandler;
use http::Server;
use std::env;

fn main() {
    let default_path = format!("{}/public", env!("CARGO_MANIFEST_DIR"));
    let public_path = env::var("PUBLIC_PATH").unwrap_or(default_path);
    let server = Server::new("127.0.0.1", 8000);

    if let Err(e) = server.run(WebHandler::new(public_path)) {
        println!("Server encountered an error! {}", e)
    }
}
