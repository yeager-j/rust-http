use std::net::TcpListener;
use std::io::Read;
use crate::http::Request;
use std::convert::TryFrom;
use anyhow::Result;

pub struct Server {
    host: String,
    port: u16
}

impl Server {
    pub fn new<S: Into<String>>(host: S, port: u16) -> Server {
        return Server {
            host: host.into(),
            port
        };
    }

    pub fn run(&self) -> Result<()> {
        println!("Listening on {}:{}", self.host, self.port);

        let listener = TcpListener::bind(&format!("{}:{}", &self.host, &self.port)).unwrap();

        loop {
            let (mut socket, addr) = listener.accept()?;
            println!("Established connection with {}:{}", addr.ip(), addr.port());

            let mut buffer = [0; 1024];
            socket.read(&mut buffer)?;
            println!("Received request! {}", String::from_utf8_lossy(&buffer));

            let req = Request::try_from(&buffer[..])?;
            println!("Method: {:?}, Query: {:?}, Path: {}", req.method, req.query, req.path);
        }
    }
}