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

    pub fn run(&self) {

    }
}