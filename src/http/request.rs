use crate::http::http_method::{HttpMethod, MethodError};
use crate::http::QueryString;
use std::convert::TryFrom;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};
use std::str;
use std::str::Utf8Error;

#[derive(Debug)]
pub struct Request<'a> {
    pub path: &'a str,
    pub query: Option<QueryString<'a>>,
    pub method: HttpMethod,
}

impl<'a> TryFrom<&'a [u8]> for Request<'a> {
    type Error = ParseError;

    fn try_from(buffer: &'a [u8]) -> Result<Self, Self::Error> {
        let request = str::from_utf8(buffer)?;
        let (method, request) = iterate_str_spaces(&request).ok_or(ParseError::InvalidRequest)?;
        let (mut path, request) = iterate_str_spaces(&request).ok_or(ParseError::InvalidRequest)?;
        let (protocol, _) = iterate_str_spaces(&request).ok_or(ParseError::InvalidRequest)?;

        if protocol != "HTTP/1.1" {
            return Err(ParseError::InvalidProtocol);
        }

        let method: HttpMethod = method.parse()?;
        let mut query = None;

        if let Some(i) = path.find('?') {
            query = Some(QueryString::from(&path[i + 1..]));
            path = &path[..i];
        }

        Ok(Self {
            path,
            query,
            method,
        })
    }
}

fn iterate_str_spaces(string: &str) -> Option<(&str, &str)> {
    for (i, c) in string.chars().enumerate() {
        if c == ' ' || c == '\r' {
            return Some((&string[..i], &string[i + 1..]));
        }
    }

    return None;
}

#[derive(Debug)]
pub enum ParseError {
    InvalidRequest,
    InvalidEncoding,
    InvalidProtocol,
    InvalidMethod,
}

impl ParseError {
    fn message(&self) -> &str {
        match self {
            Self::InvalidRequest => "Invalid Request",
            Self::InvalidEncoding => "Invalid Encoding",
            Self::InvalidProtocol => "Invalid Protocol",
            Self::InvalidMethod => "Invalid Method",
        }
    }
}

impl From<Utf8Error> for ParseError {
    fn from(_: Utf8Error) -> Self {
        return Self::InvalidEncoding;
    }
}

impl From<MethodError> for ParseError {
    fn from(_: MethodError) -> Self {
        return Self::InvalidMethod;
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

impl Error for ParseError {}
