mod geometry;
mod webserver;

use webserver::{http_method::HttpMethod, http_status::HttpStatus, response::Response, webserver::WebServer};
use crate::webserver::routehandler::RouteHandler;

use tracing::{info, error};
use tracing_subscriber::{
    prelude::*,
    fmt,
    layer::Layer,
    Registry, 
    filter
};
use std::sync::Arc;
use std::io::Write;
use std::fs;



/// The main function initializes the web server, sets up routes, and starts the server.
fn main() -> Result<(), Box<dyn std::error::Error>> {

    // Setup logging.
    setup_tracing();

    // Start the webserver on localhost 8080.
    let mut server = WebServer::new("localhost", "8080");

    // Add all routes to the server.
    for route in define_routes() {

        // Add the route to the server.
        // Log route info before moving it.
        let method = route.method.clone();
        let path = route.path.clone();
        let added = server.add_route(route);
        if !added  {
            error!( "Failed to add route: {} {}", method, path);
            std::process::exit(1);
        } else {
            info!("Added route: {} {}", method, path);
        }
    }

    // Start the server.
    server.start();

    // Wait for the user to press Enter to stop the server.
    println!("Press Enter to exit...");
    let _ = std::io::stdout().flush();
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    // Stop the server.
    info!("Stopping server...");
    server.stop();
    info!("Server stopped.");

    Ok(())
}

// Setup logging with tracing library.
fn setup_tracing() {

    // Log file access.
    let log_file = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open("log.txt")
        .unwrap();

    // Subscribe to the tracing events.
    let subcriber = Registry::default()
        .with(
            // Info messages and higher are written to the console.
            fmt::Layer::new()
                .compact()
                .with_ansi(true)
                .with_filter(filter::LevelFilter::INFO)
        )
        .with(
            // Debug messages and higher are written to the log file.
            fmt::Layer::new()
                .with_writer(log_file)
                .with_ansi(false)
                .with_filter(filter::LevelFilter::DEBUG)
        );
    
    // Apply the subscriber.
    tracing::subscriber::set_global_default(subcriber).unwrap();
}

/// Function to define all routes this web server will handle.
/// # Returns
/// A list of routes.
fn define_routes() -> Vec<RouteHandler> {

    // Create a vector to hold the routes.
    let mut routes = Vec::new();

    // Add the root route that serves the index.html file.
    routes.push(webserver::routehandler::RouteHandler::new(
        HttpMethod::GET,
        "/",
        Arc::new(|_path| {

            // Return the index.html file.
            let response_body = fs::read_to_string("src/webserver/index.html").unwrap();
            let length = response_body.len();
            let response: Response = Response::new(
                HttpStatus::Ok,
                response_body,
                vec![
                    ("Content-Type".to_string(), "text/html; charset=utf-8".to_string()),
                    ("Content-Length".to_string(), length.to_string()),
                ]
            );
            info!("Response: {}", response);
            
            return response;
        })
    ) );

    // Return the routes.
    routes
}
