use std::fmt;

pub enum HttpStatus {
    Ok,
    NotFound,
    InternalServerError,
}

impl fmt::Display for HttpStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            HttpStatus::Ok => write!(f, "200 OK"),
            HttpStatus::NotFound => write!(f, "404 Not Found"),
            HttpStatus::InternalServerError => write!(f, "500 Internal Server Error"),
        }
    }
}