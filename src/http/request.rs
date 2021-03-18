pub enum HttpMethod {
    GET,
    POST,
    PUT,
    PATCH,
    DELETE,
    OPTIONS,
    TRACE,
    CONNECT,
    HEAD
}

pub struct Request {
    path: String,
    query: Option<String>,
    method: HttpMethod
}