use std::{
    env, fs, hash::Hash, io::{prelude::*, BufReader}, net::{SocketAddr, TcpListener, TcpStream}, sync::{atomic::{AtomicBool, Ordering}, Arc}
};

use tracing::{info, debug, error};

use super::routehandler::RouteHandler;
use super::http_method::HttpMethod;
use super::http_status::HttpStatus;
use super::request::Request;
use super::response::Response;


/// Represents a simple web server.
/// It can handle routes and serve HTTP requests.
pub struct WebServer {
    routes: Vec<RouteHandler>,
    is_running: bool,
    should_stop: Arc<AtomicBool>,
    local_addr: Option<SocketAddr>,
    listener_handle: Option<std::thread::JoinHandle<()>>,
    listener: Option<TcpListener>,
    pub address: String,
    pub port: String
}

/// Implement the WebServer struct.
/// This struct represents a simple web server that can handle HTTP requests.
/// It can be started, stopped, and can handle routes.
/// It uses a TcpListener to listen for incoming connections and handles them in a separate thread.
impl WebServer {

    /// Create a new WebServer instance.
    ///
    /// # Arguments
    /// * `url` - The URL or IP address to bind the server to (e.g., "localhost").
    /// * `port` - The port to bind the server to (e.g., "8080").
    pub fn new( url: &str, port: &str ) -> WebServer {

        // Todo: Check if the address and port are valid.
        WebServer{
            routes: Vec::new(),
            address: url.to_string(), port: port.to_string(),
            is_running: false,
            should_stop: Arc::new(AtomicBool::new(false)),
            local_addr: None,
            listener_handle: None,
            listener: None
        }
    }

    /// Add a route to the web server.
    /// arguments
    /// * `handler` - The RouteHandler that will handle requests for this route.
    /// # Returns
    /// A boolean indicating whether the route was added successfully.
    pub fn add_route (
        &mut self,
        handler: RouteHandler
    ) -> bool {

        debug!("Adding route handler '{}'", handler.to_string() );

        // Check if the server is running.
        if self.is_running {
            error!("Cannot add route: Server is running. Stop the server first.");
            return false;
        }

        // Check if the handler is already registered.
        if self.routes.iter().any( |r| r.path == handler.path && r.method == handler.method ) {
            error!("Cannot add route: Handler for path '{}' with method '{}' is already registered.", handler.path, handler.method);
            return false;
        }

        // Validate the handler.
        if handler.path.is_empty() {
            error!("Cannot add route: Handler for path '{}' with method '{}' is empty.", handler.path, handler.method);
            return false;
        }
        if handler.method == HttpMethod::Unknown {
            error!("Cannot add route: Handler for path '{}' has an unknown method.", handler.path);
            return false;
        }

        // Check if the path pattern is valid.
        // For now, we only support simple paths without regex or wildcards.
        let unsupported_wildcards = [
            "{", "}", "*", "?", "+", "|", "^", "$", ".", "\\", "#", "&", "="
        ];
        // Check if path pattern has any of the unsupported widlcards.
        for wildcard in unsupported_wildcards.iter() {
            if handler.path_pattern.contains(wildcard) {
                error!("Cannot add route: Handler for path '{}' with method '{}' has a path pattern with unsupported wildcard '{}'.", handler.path, handler.method, wildcard);
                return false;
            }
        }

        // Add the handler to the list of routes.
        self.routes.push( handler);
        return true;
    }

    
    /// Start the web server.
    pub fn start(&mut self) {

        // Check if the server is already running.
        if self.is_running {
            info!( "Start: Server is already running." );
            return;
        }
        
        // Start listening for incoming connections. TODO: error handling
        self.listener = Some( TcpListener::bind( format!("{}:{}", self.address, self.port) ).unwrap() );
        let socket_addr = self.listener.as_ref().unwrap().local_addr();
        match socket_addr {
            Ok(addr) => {
                self.local_addr = Some(addr);
            }
            Err(e) => {
                error!("Error: {}", e);
            }
        }
        self.is_running = true;
        self.should_stop.store(false, Ordering::Relaxed);
        debug!("Created listener on {}:{}", self.address, self.port);

        // Start handling incoming connections in a separate thread.
        // We need to take the listener out of the Option so we can move it into the thread.        
        let listener = self.listener.take().unwrap();
        let routes = self.routes.clone();
        let should_stop = self.should_stop.clone();
        self.listener_handle = Some(std::thread::spawn(move || {
            
            // Loop to handle incoming connections.
            info!("Listening for incoming connections on {}...", listener.local_addr().unwrap());
            for stream in listener.incoming() {
                
                // Check if the server should stop.
                if should_stop.load(Ordering::Relaxed) {
                    info!("Server should stop!");
                    break;
                }

                // Handle the incoming connection.
                match stream {
                    Ok(stream) => {
                        info!("Request arrived.");
                        handle_connection(stream, &routes);
                    }
                    Err(e) => {
                        error!("Error: {}", e);
                    }
                }
            }
        }));
 
    }

    /// Stop the web server.
    pub fn stop(&mut self) {
        
        // Check if the server is running.
        if !self.is_running {
            info!( "Stop: Server is not running." );
            return;
        }

        // Check if listener handle is set.
        if self.listener_handle.is_none() {
            error!( "Stop: Listener handle is not set, cannot stop!" );
            return;
        }

        // Check if local address is set.
        if self.local_addr.is_none() {
            error!( "Stop: Local address is not set, cannot stop!" );
            return;
        }

        // Stop request handler loop the next time there is a request.
        self.should_stop.store(true, Ordering::Relaxed);

        // Call the listner to unblock it.
        let _ = TcpStream::connect( self.local_addr.unwrap() );

        // Wait for the listener to finish.
        if let Some(handle) = self.listener_handle.take() {
            let _ = handle.join();
        }

        // Server is stopped.
        self.is_running = false;
    }


}

/// Handle an incoming connection.
/// This function reads the request from the stream, parses it, finds the appropriate route handler,
/// and sends the response back to the client.
/// This function is called in a separate thread for each incoming connection.
fn handle_connection(
    mut stream: std::net::TcpStream,
    routes: &Vec<RouteHandler>,
) {

    // Read the request line by line from the buffer to a vector.
    let buf_reader = std::io::BufReader::new(&stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map( |result| result.unwrap() ) // todo: error handling
        .take_while( |line| !line.is_empty() )
        .collect();

    debug!("Got request raw: {http_request:#?}");

    // Wrap the request in a Request struct.
    let request = Request::new( &http_request );
    info!("Request: '{}'", request.to_string());
    
    // Find the route handler for the path.
    let route_handler = routes.iter().find( 
            |route| route.handles_path( request.method, &request.path ) );

    // If no route handler was found, return a 404. otherwise, call the handler.
    let response: Response;
    if route_handler.is_none() {
        info!("No route handler found for request '{} {}'", request.method.to_string(), request.path);
        response = Response::new(
        HttpStatus::NotFound,
        r#"<!DOCTYPE html>
                <html lang="en">
                <head>
                    <meta charset="UTF-8">
                    <title>404 Not Found</title>
                    <style>
                        body { font-family: sans-serif; background: #f8f8f8; color: #333; text-align: center; margin-top: 10%; }
                        h1 { font-size: 3em; margin-bottom: 0.2em; }
                        p { font-size: 1.2em; }
                    </style>
                </head>
                <body>
                    <h1>404 Not Found</h1>
                    <p>The page you requested could not be found.</p>
                </body>
                </html>
                "#.to_string(),
        vec![("Content-Type".to_string(), "text/html; charset=utf-8".to_string())]
        );
        info!("Response: {}", response.to_string());
        debug!("No route handler found for request '{} {}'", request.method.to_string(), request.path);
    }
    else {

        // Get the handler.
        let handler = route_handler.unwrap();

        info!("Using route handler '{}' for request '{} {}'", 
            handler.to_string(), 
            request.method.to_string(), 
            request.path);
        
        // Call the route handler.
        // We need to clone the handler because it is an Arc, and we need to move it into the closure 
        // to avoid borrowing issues.
        let handler_arc = handler.handler.clone();
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe( move || {
            (handler_arc)(request)
        }));
        response = match result {
            Ok(resp) => resp,
            Err(panic_info) => {
                // Try to extract the panic message
                if let Some(s) = panic_info.downcast_ref::<&str>() {
                    debug!("Handler panicked: {}", s);
                } else if let Some(s) = panic_info.downcast_ref::<String>() {
                    debug!("Handler panicked: {}", s);
                } else {
                    debug!("Handler panicked with unknown cause.");
                } 
            
                // Return a 500 Internal Server Error response.
                Response::new(
                HttpStatus::InternalServerError,
                r#"<!DOCTYPE html>
                    <html lang="en">
                    <head>
                        <meta charset="UTF-8">
                        <title>500 Internal Server Error</title>
                        <style>
                            body { font-family: sans-serif; background: #f8f8f8; color: #333; text-align: center; margin-top: 10%; }
                            h1 { font-size: 3em; margin-bottom: 0.2em; }
                            p { font-size: 1.2em; }
                        </style>
                    </head>
                    <body>
                        <h1>500 Internal Server Error</h1>
                        <p>The server encountered an unexpected condition.</p>
                    </body>
                    </html>
                    "#.to_string(),
                vec![("Content-Type".to_string(), "text/html; charset=utf-8".to_string())]
            ) },
        };
        info!("Response from handler: {}", response.to_string());
    }
    
    // Write the response to the stream.
    stream.write_all(response.to_string().as_bytes()).unwrap(); // todo: error handling

}