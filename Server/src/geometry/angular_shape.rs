use crate::geometry::point::Point;
use crate::geometry::line::Line;
use crate::geometry::traits::AreaCalculatable;
use std::error::Error;

use super::line::LineError;

pub struct AngularShape {
    pub corners: Vec<Point>
}

// Define the error types.
#[derive(Debug)]
pub enum ShapeError {
    NotValidShape( Box<dyn Error> ),
}
impl std::fmt::Display for ShapeError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ShapeError::NotValidShape( error ) => {
                write!(f, "The shape is not valid. {}", error)
            }
        }
    }
}
impl Error for ShapeError {}
impl From<LineError> for ShapeError {
    fn from( error: LineError ) -> Self {
        ShapeError::NotValidShape( Box::new( error ) )
    }
}

// Implement the AngularShape struct.
impl AngularShape {

    // Check if the shape is valid.
    pub fn is_valid( &self ) -> Result<bool,  ShapeError> {

        // A shape with less than 3 corner points is not a valid shape.
        if self.corners.len() < 3 {
            print!(" Shape is not valid, less than 3 corner points!");
            return Ok(false);
        }

        // A shape is not valid if the points are the same.
        for i in 0..self.corners.len() {
            for j in 0..self.corners.len() {
                if i != j && self.corners[i] == self.corners[j] {
                    println!(" Shape is not valid, corner points {i} and {j} are the same {}!", self.corners[i]);
                    return Ok(false);
                }
            }
        }

        // Check that the starting and ending points of the subsequent lines are the same.
        let lines: Vec<Line> = self.get_lines();
        for i in 0..lines.len() {
            
            // Skip the last line as it is automatically connected to the first line.
            if i == lines.len() - 1 {
                break;
            }

            // Shape is not valid if the lines do not connect.
            if lines[i].end != lines[i + 1].start {
                println!(" Shape is not valid, lines do not connect! End point of a line {} ({}) is not the same as the start point of the next line ({}) in the shape!", 
                    i, lines[i].end, lines[i + 1].start);
                return Ok(false);
            }
        }

        // The lines should no intersect each other.
        for i in 0..lines.len() {

            // Move the points a little bit as the
            // start and end point should be the same, and thus the lines
            // actually intersect every time.
            let adjusted_line: Line = lines[i]
                .shorten( 0.000001 )?
                .flip()
                .shorten( 0.000001)?
                .flip();

            // Move the start point and end point a little bit.

            // Compare this adjusted line to the other lines.
            for j in 0..lines.len() {
                if i != j {


                    // Check if the lines intersect.
                    if adjusted_line.intersects( &lines[j] ) {
                        println!(" Shape is not valid, lines intersect! Line {i} intersects with line {j} in the shape!");
                        return Ok(false);
                    }
                }
            }
        }

        // Area should be valid.
        return Ok(true);
        
    }

    // Get the lines that make up the shape.
    pub fn get_lines( &self ) -> Vec<Line> {

        // Create lines between the points.
        // The last point should be connected to the first point.
        let mut lines: Vec<Line> = Vec::new();
        for i in 0..self.corners.len() {
            let start = self.corners[i];
            let end = self.corners[ (i + 1) % self.corners.len() ];
            lines.push( Line { start, end } );
        }
        
        // Return the lines.
        return lines;
    }
}

// Implement the AreaCalculatable trait for the AngularShape struct.
impl AreaCalculatable for AngularShape {

    // https://www.mathsisfun.com/geometry/area-irregular-polygons.html
    // Returns the area of the shape, or -1 if the shape is not valid.
    fn area(&self) -> f64 {
        
        // Check if the shape is valid. Return -1 if the shape is not valid.
        if !self.is_valid().unwrap() {
            return -1.0;
        }

        // Calcluate areas of between the lines and the x-axis
        // and then sum then to get the area of the shape.
        let mut areas: Vec<f64> = Vec::new();

        // Get the lines that make up the shape.
        let lines = self.get_lines();

        // Calculate the area between the lines and the x-axis.
        for line in lines {

            // Calculate the average height of the line.
            let height: f64 = (line.start.y + line.end.y) / 2.0;

            // Calculate the line lenght in x-direction.
            let dx: f64 = line.end.x - line.start.x;

            // Calculate the area of the trapezoid.
            let area_trapezoid = height * dx;
            areas.push( area_trapezoid);
        }

        // Sum the areas of the trapezoids to get the area of the shape.
        let result: f64 = areas.iter().sum();

        // Return the the absolute value of the result. The sign of the result
        // depend on the direction of which the lines are drawn.
        return result.abs()
    }

}



// Unit tests for AngularShape.
#[cfg(test)]
mod tests {
    use std::result::Result;

    use super::*;


    #[test]
    fn test_get_lines() {
        let square: AngularShape = AngularShape {
            corners: vec![
                Point { x: 0.0, y: 0.0 },
                Point { x: 0.0, y: 2.0 },
                Point { x: 2.0, y: 2.0 },
                Point { x: 2.0, y: 0.0 },
            ]
        };
        let lines: Vec<Line> = square.get_lines();
        assert!( lines.len() == 4 );

        // Assert line start and end points.
        for i in 0..lines.len() {
            assert!( lines[i].start == square.corners[i] );
            assert!( lines[i].end == square.corners[ (i + 1) % square.corners.len() ] );
        }
    }

    #[test]
    fn test_get_lines_zero() {
        let square: AngularShape = AngularShape {
            corners: vec![]
        };
        let lines: Vec<Line> = square.get_lines();
        assert!( lines.len() == 0 );
    }

    #[test]
    fn get_lines_one_corner() {
        let square: AngularShape = AngularShape {
            corners: vec![
                Point { x: 0.0, y: 0.0 }
            ]
        };
        let lines: Vec<Line> = square.get_lines();
        assert!( lines.len() == 1 );
        assert!( lines[0].start == square.corners[0] );
        assert!( lines[0].end == square.corners[0] );
    }

    #[test]
    fn test_is_valid_triangle () -> Result<(), Box<dyn Error>> {
        let square: AngularShape = AngularShape {
            corners: vec![
                Point { x: 0.0, y: 0.0 },
                Point { x: 0.0, y: 2.0 },
                Point { x: 2.0, y: 2.0 }
            ]
        };
        assert!( square.is_valid()? );
        Ok(())
    }

    #[test]
    fn test_is_valid_square () -> Result<(), Box<dyn Error>> {
        let square: AngularShape = AngularShape {
            corners: vec![
                Point { x: 0.0, y: 0.0 },
                Point { x: 0.0, y: 2.0 },
                Point { x: 2.0, y: 2.0 },
                Point { x: 2.0, y: 0.0 },
            ]
        };
        assert!( square.is_valid()? );
        Ok(())
    }

    #[test]
    fn test_is_not_valid_line () -> Result<(), Box<dyn Error>> {
        let square: AngularShape = AngularShape {
            corners: vec![
                Point { x: 0.0, y: 0.0 },
                Point { x: 0.0, y: 2.0 }
            ]
        };
        assert!( square.is_valid()? == false );
        Ok(())
    }

    #[test]
    fn test_is_not_valid_same_points () -> Result<(), Box<dyn Error>> {
        let square: AngularShape = AngularShape {
            corners: vec![
                Point { x: 0.0, y: 0.0 },
                Point { x: 0.0, y: 2.0 },
                Point { x: 2.0, y: 2.0 },
                Point { x: 2.0, y: 1.0 },
                Point { x: 0.0, y: 2.0 },
                Point { x: 0.5, y: 0.5 },
            ]
        };
        assert!( square.is_valid()? == false );
        Ok(())
    }

    #[test]
    fn test_is_not_valid_dot() -> Result<(), Box<dyn Error>> {
        let square: AngularShape = AngularShape {
            corners: vec![
                Point { x: 0.0, y: 0.0 }
            ]
        };
        assert!( square.is_valid()? == false );
        Ok(())
    }

    #[test]
    fn test_is_not_valid_empty() -> Result<(), Box<dyn Error>> {
        let square: AngularShape = AngularShape {
            corners: vec![]
        };
        assert!( square.is_valid()? == false );
        Ok(())
    }

    #[test]
    fn test_is_not_valid_intersects() -> Result<(), Box<dyn Error>> {
        let square: AngularShape = AngularShape {
            corners: vec![
                Point { x: 0.0, y: 0.0 },
                Point { x: 0.0, y: 2.0 },
                Point { x: 2.0, y: 2.0 },
                Point { x: 2.0, y: 3.0 },
            ]
        };
        assert!( square.is_valid()? == false );
        Ok(())
    }

    #[test]
    fn test_triangle_area() -> Result<(), Box<dyn Error>> {
        let triangle: AngularShape = AngularShape {
            corners: vec![
                Point { x: 0.0, y: 0.0 },
                Point { x: 0.0, y: 2.0 },
                Point { x: 2.0, y: 0.0 }
            ]
        };
        let area: f64 = triangle.area();
        assert!( area >= 0.0, "Area should always be positive or zero." );
        assert!( (triangle.area() - 2.0).abs() < 0.000001 );
        Ok(())
    }

    #[test]
    fn test_square_area() {
        let square: AngularShape = AngularShape {
            corners: vec![
                Point { x: 0.0, y: 0.0 },
                Point { x: 0.0, y: 2.0 },
                Point { x: 2.0, y: 2.0 },
                Point { x: 2.0, y: 0.0 },
            ]
        };
        let area: f64 = square.area();
        assert!( area > 0.0, "Area should always be positive." );
        assert!( (square.area() - 4.0).abs() < 0.000001 );
    }

    #[test]
    fn test_pentagon_area() {
        let pentagon: AngularShape = AngularShape {
            corners: vec![
                Point { x: 0.0, y: 0.0 },
                Point { x: -1.0, y: 1.0 },
                Point { x: 0.5, y: 2.0 },
                Point { x: 1.5, y: 1.0 },
                Point { x: 1.0, y: 0.0 },
            ]
        };
        let area: f64 = pentagon.area();
        assert!( area > 0.0, "Area should always be positive." );
        assert!( (pentagon.area() - 3.0).abs() < 0.000001 );
    }

    #[test]
    fn test_not_valid_area_() {
        let square: AngularShape = AngularShape {
            corners: vec![
                Point { x: 0.0, y: 0.0 },
                Point { x: 0.0, y: 2.0 },
                Point { x: 2.0, y: 2.0 },
                Point { x: 2.0, y: 3.0 },
            ]
        };
        assert!( square.area() == -1.0 );
    }


}