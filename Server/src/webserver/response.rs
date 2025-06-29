use std::fmt;

use super::http_status::HttpStatus;

pub struct Response {
    pub headers: Vec<(String, String)>,
    pub status: HttpStatus,
    pub body: String
}

impl Response {
    pub fn new( status: HttpStatus, body: String) -> Response {
        Response {
            headers: vec![("Content-Type".to_string(), "text/html".to_string())],
            status,
            body
        }
    }

    pub fn to_string(&self) -> String {
        let status_line = format!("HTTP/1.1 {status}", status=self.status);
        let headers = self.headers.iter().map(|(key, value)| format!("{key}: {value}", key=key, value=value)).collect::<Vec<_>>().join("\r\n");
        format!("{status_line}\r\n{headers}\r\n\r\n{body}", status_line=status_line, headers=headers, body=self.body)
    }
}

impl fmt::Display for Response {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{status}", status=self.status)
    }
}