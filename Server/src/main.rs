mod geometry;
mod webserver;

use webserver::{http_method::HttpMethod, http_status::HttpStatus, response::Response, webserver::WebServer};

use tracing::{info};
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



fn main() -> Result<(), Box<dyn std::error::Error>> {

    // Setup logging.
    setup_tracing();

    // Start the webserver on localhost 8080.
    let mut server = WebServer::new("localhost", "8080");

    // Add a route to the server.
    server.add_route( 
        webserver::routehandler::RouteHandler::new(
        HttpMethod::GET,
        "/",
        Arc::new(|_path| {

            //return Response::new( HttpStatus::Ok, String::new() );

            // Return the index.html file.
            
            let response_body = fs::read_to_string("src/webserver/index.html").unwrap();
            let length = response_body.len();
            let header_content_length = format!( "Content-Length: {length}");

            /* let response = format!(
                "{response_status}\r\n{content_length}\r\n\r\n{response_body}"
            ); */
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
    ));

    server.start();

    println!("Press Enter to exit...");
    let _ = std::io::stdout().flush();
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

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
