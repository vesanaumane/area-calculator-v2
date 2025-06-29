use std::fmt;

/// This module defines the HTTP status codes used in the web server.
pub enum HttpStatus {
    Ok,
    NotFound,
    InternalServerError,
}

/// Implement the Display trait for HttpStatus to allow easy printing.
impl fmt::Display for HttpStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            HttpStatus::Ok => write!(f, "200 OK"),
            HttpStatus::NotFound => write!(f, "404 Not Found"),
            HttpStatus::InternalServerError => write!(f, "500 Internal Server Error"),
        }
    }
}