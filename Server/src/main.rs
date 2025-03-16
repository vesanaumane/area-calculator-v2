mod geometry;

use crate::geometry::line::Line;
use crate::geometry::point::Point;
use crate::geometry::angular_shape::AngularShape;
use crate::geometry::traits::AreaCalculatable;


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let line1: Line = Line {
        start: Point { x: 1.0, y: 1.0 },
        end: Point { x: 5.0, y: 1.0 },
    };

    let line2: Line = Line {
        start: Point { x: 1.0, y: 1.2 },
        end: Point { x: 5.0, y: 5.0 },
    };

    println!("The length of the line1 is: {}", line1.length() );
    println!("The length of the line2 is: {}", line2.length() );

    if line1.intersects( &line2 ) {
        println!("The lines intersect.");
    } else {
        println!("The lines do not intersect.");
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
        println!("The square is valid.");
    } else {
        println!("The square is not valid.");
    }

    // Calculate the area of the square.
    println!("The area of the square is: {}", square.area() );

    Ok(())
}
