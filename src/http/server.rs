use crate::http::request::ParseError;
use crate::http::{Request, Response, StatusCode};
use anyhow::Result;
use std::convert::TryFrom;
use std::io::Read;
use std::net::TcpListener;

pub trait Handler {
    fn handle_request(&mut self, request: &Request) -> Response;

    fn handle_bad_request(&mut self, e: &ParseError) -> Response {
        println!("Failed to parse request! {}", e);
        Response::new(StatusCode::BadRequest, None)
    }
}

pub struct Server {
    host: String,
    port: u16,
}

impl Server {
    pub fn new<S: Into<String>>(host: S, port: u16) -> Server {
        return Server {
            host: host.into(),
            port,
        };
    }

    pub fn run(&self, mut handler: impl Handler) -> Result<()> {
        println!("Listening on {}:{}", self.host, self.port);

        let listener = TcpListener::bind(&format!("{}:{}", &self.host, &self.port)).unwrap();

        loop {
            let (mut socket, addr) = listener.accept()?;

            println!("Established connection with {}:{}", addr.ip(), addr.port());

            let mut buffer = [0; 1024];
            socket.read(&mut buffer)?;

            println!("Received request! {}", String::from_utf8_lossy(&buffer));

            let response = match Request::try_from(&buffer[..]) {
                Ok(req) => {
                    println!(
                        "Method: {:?}, Query: {:?}, Path: {}",
                        req.method, req.query, req.path
                    );

                    handler.handle_request(&req)
                }
                Err(e) => handler.handle_bad_request(&e),
            };

            if let Err(e) = response.send(&mut socket) {
                println!("Failed to send response! {}", e);
            }
        }
    }
}
