use std::fmt;
use std::str::FromStr;

/// This enum represents the HTTP methods used in the web server.
/// It includes common methods like GET, POST, PUT, DELETE, and an Unknown variant for unrecognized methods.
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum HttpMethod {
    GET,
    POST,
    PUT,
    DELETE,
    Unknown
}

/// This trait allows the HttpMethod to be printed as a string.
/// It is used to convert the HttpMethod enum into a string representation that can be used in HTTP requests and responses.
impl fmt::Display for HttpMethod {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            HttpMethod::GET => write!(f, "GET"),
            HttpMethod::POST => write!(f, "POST"),
            HttpMethod::PUT => write!(f, "PUT"),
            HttpMethod::DELETE => write!(f, "DELETE"),
            HttpMethod::Unknown => write!(f, "UNKNOWN"),
        }
    }
}

/// This trait allows the HttpMethod to be parsed from a string.
/// It is used to convert string representations of HTTP methods (like "GET", "POST", etc.) into the HttpMethod enum.
impl FromStr for HttpMethod {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "GET" => Ok(HttpMethod::GET),
            "POST" => Ok(HttpMethod::POST),
            "PUT" => Ok(HttpMethod::PUT),
            "DELETE" => Ok(HttpMethod::DELETE),
            _ => Err(())
        }
    }
}

/// This trait allows the HttpMethod to be used as a key in a HashMap.
/// Default implementation is provided to return `HttpMethod::Unknown`.
impl Default for HttpMethod {
    fn default() -> Self {
        HttpMethod::Unknown
    }
}