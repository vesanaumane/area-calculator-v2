use std::fmt;

use super::http_status::HttpStatus;

/// Represents an HTTP response.
pub struct Response {
    pub headers: Vec<(String, String)>,
    pub status: HttpStatus,
    pub body: String
}

/// This struct represents an HTTP response.
/// It contains the status, headers, and body of the response.
/// It can be converted to a string representation that can be sent over the network.
impl Response {

    /// Creates a new Response with the specified status, body, and headers.
    /// 
    /// # Arguments
    /// * `status` - The HTTP status of the response (e.g., 200 OK, 404 Not Found).
    /// * `body` - The body of the response, which is a string.
    /// * `headers` - A vector of tuples representing the headers of the response, where each tuple contains a key and a value.
    pub fn new( 
        status: HttpStatus, 
        body: String, 
        headers: Vec<(String, String)>) -> Response {
        Response {
            headers,
            status,
            body
        }
    }

    /// Converts the response to a string representation.
    /// This string can be sent over the network as an HTTP response.
    /// # Returns
    /// A string representation of the HTTP response.
    pub fn to_string(&self) -> String {
        let status_line = format!("HTTP/1.1 {status}", status=self.status);
        let headers = self.headers.iter().map(|(key, value)| format!("{key}: {value}", key=key, value=value)).collect::<Vec<_>>().join("\r\n");
        format!("{status_line}\r\n{headers}\r\n\r\n{body}", status_line=status_line, headers=headers, body=self.body)
    }
}

//// Implement the Display trait for Response to allow easy printing.
impl fmt::Display for Response {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{status}", status=self.status)
    }
}