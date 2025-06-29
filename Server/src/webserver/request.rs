use std::{fmt, str::FromStr};

use tracing::info;

use super::http_method::HttpMethod;

/// Represents an HTTP request.
pub struct Request {
    pub method: HttpMethod,
    pub path: String,
    pub headers: Vec<(String, String)>,
    pub body: String
}

/// This struct represents an HTTP request.
/// It contains the method, path, headers, and body of the request.
/// It can be parsed from a raw request string.
impl Request {

    /// Creates a new Request from a raw request string.
    /// 
    /// # Arguments
    /// * `raw_request` - A vector of strings representing the raw request, where the first element is the request line and the subsequent elements are headers.
    pub fn new( raw_request: &Vec<String> ) -> Request {

        // Parse the path and method.
        let first_line = raw_request[0].split_whitespace().collect::<Vec<_>>();
        let method = first_line[0];
        let path = first_line[1];

        // Parse the method.
        let http_method = HttpMethod::from_str(method).unwrap();

        // Parse headers.
        let mut headers = Vec::new();
        for header in raw_request.iter().skip(1) {
            let header_parts = header.split(":").collect::<Vec<_>>();
            headers.push( (header_parts[0].to_string(), header_parts[1].to_string()) );
        }

        // TODO: Parse the body.

        // Create and return the Request object.
        Request {
            method: http_method,
            path: path.to_string(),
            headers: headers,
            body: String::new()
        }
    }
}

/// Implement the Display trait for Request to allow easy printing.
impl fmt::Display for Request {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{method} {path}", method=self.method, path=self.path)
    }
}