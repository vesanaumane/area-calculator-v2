mod geometry;

use crate::geometry::line::Line;
use crate::geometry::point::Point;
use crate::geometry::angular_shape::AngularShape;
use crate::geometry::traits::AreaCalculatable;

use tracing::{info};
use tracing_subscriber::{
    prelude::*,
    fmt,
    layer::Layer,
    Registry, 
    filter
};


fn main() -> Result<(), Box<dyn std::error::Error>> {

    // Setup logging.
    setup_tracing();
    info!("Server started.");

    let line1: Line = Line {
        start: Point { x: 1.0, y: 1.0 },
        end: Point { x: 5.0, y: 1.0 },
    };

    let line2: Line = Line {
        start: Point { x: 1.0, y: 1.2 },
        end: Point { x: 5.0, y: 5.0 },
    };

    info!("The length of the line1 is: {}", line1.length() );
    info!("The length of the line2 is: {}", line2.length() );

    if line1.intersects( &line2 ) {
        info!("The lines intersect.");
    } else {
        info!("The lines do not intersect.");
    }

    // Create a square. The first point is also the last point.
    let square: AngularShape = AngularShape {
        corners: vec![
            Point { x: 0.0, y: 0.0 },
            Point { x: 0.0, y: 2.0 },
            Point { x: 2.0, y: 2.0 }
        ]
    };

    // Check if the square is valid.
    if square.is_valid()? {
        info!("The square is valid.");
    } else {
        info!("The square is not valid.");
    }

    // Calculate the area of the square.
    info!("The area of the square is: {}", square.area() );

    info!("Server stopped.");

    Ok(())
}

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
