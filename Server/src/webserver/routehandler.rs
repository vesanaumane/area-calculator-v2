use std::fmt;
use std::sync::Arc;

use super::http_method::HttpMethod;
use super::request::Request;
use super::response::Response;


/// Helper trait for cloning trait objects
pub trait HandlerClone {
    fn clone_box(&self) -> Box<dyn HandlerFn>;
}

/// Implementing Clone for Box<dyn HandlerFn>
impl<F> HandlerClone for F
where
    F: HandlerFn + Clone + 'static,
{
    fn clone_box(&self) -> Box<dyn HandlerFn> {
        Box::new(self.clone())
    }
}

/// Represents a route handler in the web server.
/// It contains the HTTP method, path, path pattern, and the handler function.
pub struct RouteHandler {
    pub method: HttpMethod,
    pub path: String,
    pub path_pattern: String,
    pub handler: Arc<dyn HandlerFn>,
}

/// Implement the RouteHandler struct.
impl RouteHandler {

    /// Creates a new RouteHandler with the specified HTTP method, path, and handler function.
    /// 
    /// # Arguments
    /// * `method` - The HTTP method (GET, POST, etc.) that this handler will respond to.
    /// * `path` - The path that this handler will respond to.
    /// * `handler` - The handler function that will be called when this route is matched.
    pub fn new(
        method: HttpMethod,
        path: &str,
        handler: Arc<dyn HandlerFn>,
    ) -> RouteHandler {
        
        // Todo: Add syntax for rest resources, like {id}, and construct a regex for it, preferably
        // with named groups, so the handler can access the resorouce ids by name.
        RouteHandler {
            method: method,
            path: path.to_string(),
            path_pattern: path.to_string(),
            handler: handler,
        }
    }

    /// Checks if this route handler can handle the given HTTP method and path.
    /// 
    /// # Arguments
    /// * `method` - The HTTP method to check against this handler.
    /// * `path` - The path to check against this handler.
    /// 
    /// # Returns
    /// * `true` if this handler can handle the method and path, `false` otherwise.
    pub fn handles_path(&self, method: HttpMethod, path: &str) -> bool {
        
        // Check if the method matches.
        if self.method != method {
            return false;
        }

        // Check if the path matches. TODO: Use regex and path_pattern.
        return self.path == path;
    }
}

/// Implement the Display trait for RouteHandler to allow easy printing.
impl fmt::Display for RouteHandler {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{method} {path}",
            method = self.method.to_string(),
            path = self.path
        )
    }
}

/// Implement Clone for RouteHandler to allow cloning of route handlers.
impl Clone for RouteHandler {
    fn clone(&self) -> Self {
        RouteHandler {
            method: self.method,
            path: self.path.clone(),
            path_pattern: self.path_pattern.clone(),
            handler: Arc::clone(&self.handler),
        }
    }
}

pub trait HandlerFn: Fn(Request) -> Response + Send + Sync {}
impl<T> HandlerFn for T where T: Fn(Request) -> Response + Send + Sync {}