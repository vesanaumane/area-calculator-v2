use std::fmt;
use std::sync::Arc;
use crate::webserver::request;
use crate::webserver::response;

// Helper trait for cloning trait objects
pub trait HandlerClone {
    fn clone_box(&self) -> Box<dyn HandlerFn>;
}

impl<F> HandlerClone for F
where
    F: HandlerFn + Clone + 'static,
{
    fn clone_box(&self) -> Box<dyn HandlerFn> {
        Box::new(self.clone())
    }
}

use super::http_method::HttpMethod;
use super::request::Request;
use super::response::Response;

pub struct RouteHandler {
    pub method: HttpMethod,
    pub path: String,
    pub path_pattern: String,
    pub handler: Arc<dyn HandlerFn>,
}

impl RouteHandler {
    pub fn new(
        method: HttpMethod,
        path: &str,
        handler: Arc<dyn HandlerFn>,
    ) -> RouteHandler {
        
        // Todo: Add syntax for rest resources, like {id}, and construct a regex for it, preferably
        // with named groups, so the handler can access the resrouce ids by name.
        RouteHandler {
            method: method,
            path: path.to_string(),
            path_pattern: path.to_string(),
            handler: handler,
        }
    }

    pub fn handles_path(&self, method: HttpMethod, path: &str) -> bool {
        // Check if the method mathces.
        if self.method != method {
            return false;
        }

        
        // Check if the path matches. TODO: Use regex and path_pattern.
        return self.path == path;
    }
}

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