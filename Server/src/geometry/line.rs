use std::error::Error;

use crate::geometry::point::Point;

// A line is defined by two points.
pub struct Line {
    pub start: Point,
    pub end: Point,
}

// Results will have LineError as error type.
type Result<T> = std::result::Result<T, LineError>;

// Define the error types.
#[derive(Debug)]
pub enum LineError {
    LineTooShortForShortening {
        current_length: f64,
        requested_shortening: f64
    }
}
impl std::fmt::Display for LineError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            LineError::LineTooShortForShortening { current_length, requested_shortening } => {
                write!(f, "The line is too short for shortening. The current length is {} and the requested shortening is {}.", current_length, requested_shortening)
            }
        }
    }
}
impl Error for LineError {
    
}

// Implement the Line struct.
impl Line {

    // Get the length of the line
    pub fn length( &self ) -> f64 {
        let dx = self.start.x - self.end.x;
        let dy = self.start.y - self.end.y;
        ( (dx * dx + dy * dy)  as f64).sqrt()
    }

    // Return a flipped version of the line.
    pub fn flip( &self ) -> Line {
        return Line {
            start: self.end.clone(),
            end: self.start.clone(),
        }
    }

    // Return a shortened version of the line.
    pub fn shorten( &self, delta_l: f64 ) -> Result<Line> {
        
        // Do not shorten if the line is too short.
        let length: f64 = self.length();
        if self.length() < delta_l {
            return Err( LineError::LineTooShortForShortening {
                current_length: length,
                requested_shortening: delta_l
            } );
        }

        // Make shorter variable names.
        let x1: f64 = self.start.x;
        let x2: f64 = self.end.x;
        let y1: f64 = self.start.y;
        let y2: f64 = self.end.y;

        // Calculate delta for x and y.
        let dx: f64 = ( x2 - x1 ) / ( x2.powi(2) - 2.0 * x2 * x1 + x1.powi(2) + y2.powi(2) - 2.0 * y2 * y1 + y1.powi(2) ).sqrt();
        let dy: f64 = ( y2 - y1 ) / ( x2.powi(2) - 2.0 * x2 * x1 + x1.powi(2) + y2.powi(2) - 2.0 * y2 * y1 + y1.powi(2) ).sqrt();

        // Keep the starting point as is, modify end point.
        let new_end: Point = Point{ x: x2 - delta_l * dx, y: y2 - delta_l * dy };

        // Return the new line.
        return Ok( Line {
            start: self.start.clone(),
            end: new_end,
        } );
    }

    // Check if the line intersects with another line.
    // https://www.geeksforgeeks.org/check-if-two-given-line-segments-intersect/
    pub fn intersects( &self, other: &Line ) -> bool {
        
        // Find the 4 orientations required for
        // the general and special cases
        let o1: i32 = self.orientation( self.start, self.end, other.start );
        let o2: i32 = self.orientation( self.start, self.end, other.end );
        let o3: i32 = self.orientation( other.start, other.end, self.start );
        let o4: i32 = self.orientation( other.start, other.end, self.end );

        // General case
        if o1 != o2 && o3 != o4 {
            return true;
        }

        // Special Cases
        // self.start, self.end and other.start are colinear and other.start lies on segment self.start, self.end
        if o1 == 0 && self.on_segment( self.start, other.start, self.end ) {
            return true;
        }

        // self.start, self.end and other.end are colinear and other.end lies on segment self.start, self.end
        if o2 == 0 && self.on_segment( self.start, other.end, self.end ) {
            return true;
        }

        // other.start, other.end and self.start are colinear and self.start lies on segment other.start, other.end
        if o3 == 0 && self.on_segment( other.start, self.start, other.end ) {
            return true;
        }

        // other.start, other.end and self.end are colinear and self.end lies on segment other.start, other.end
        if o4 == 0 && self.on_segment( other.start, self.end, other.end ) {
            return true;
        }

        return false;
    }

    // To find orientation of ordered triplet (p, q, r).
    // The function returns following values
    // 0 --> p, q and r are collinear
    // 1 --> Clockwise
    // 2 --> Counterclockwise
    fn orientation( &self, p: Point, q: Point, r: Point ) -> i32 {

        // See https://www.geeksforgeeks.org/orientation-3-ordered-points/
        // for details of below formula.
        let val: f64 = (q.y - p.y) * (r.x - q.x) - (q.x - p.x) * (r.y - q.y);
        if val == 0.0 {
            return 0; // collinear
        }
        return if val > 0.0 { 1 } else { 2 }; // clock or counterclock wise
    }

    // Given three collinear points p, q, r, the function checks if
    // point q lies on line segment 'pr'.
    fn on_segment( &self, p: Point, q: Point, r: Point ) -> bool {
        if q.x <= p.x.max(r.x) && q.x >= p.x.min(r.x) &&
            q.y <= p.y.max(r.y) && q.y >= p.y.min(r.y) {
            return true;
        }
        return false;
    }
}




// Unit tests for Line.
#[cfg(test)]
mod tests {
    use std::result::Result;

    use super::*;

    #[test]
    fn test_length() {
        let line: Line = Line {
            start: Point { x: 1.0, y: 1.0 },
            end: Point { x: 5.0, y: 1.0 },
        };
        assert_eq!( line.length(), 4.0 );
    }

    #[test]
    fn test_length_zero() {
        let line: Line = Line {
            start: Point { x: 1.0, y: 1.0 },
            end: Point { x: 1.0, y: 1.0 },
        };
        assert_eq!( line.length(), 0.0 );
    }

    #[test]
    fn test_length_zero_origo() {
        let line: Line = Line {
            start: Point { x: 0.0, y: 0.0 },
            end: Point { x: 0.0, y: 0.0 },
        };
        assert_eq!( line.length(), 0.0 );
    }

    #[test]
    fn test_length_negative() {
        let line: Line = Line {
            start: Point { x: 0.0, y: 0.0 },
            end: Point { x: -2.0, y: 0.0 },
        };
        assert_eq!( line.length(), 2.0 );
    }

    #[test]
    fn test_shorten() {

        // Create a line with length 2.
        let line: Line = Line {
            start: Point { x: 0.0, y: 0.0 },
            end: Point { x: 0.0, y: 2.0 },
        };

        // Shorten the line by 1.
        let shortened_line: Result<Line, LineError>  = line.shorten( 1.0 );
        match shortened_line {
            Ok( shortened_line ) => {
                assert_eq!( shortened_line.length(), 1.0 );
            },
            Err( _ ) => {
                assert!( false, "The line should be able to be shortened." );
            }
        }
    }

    #[test]
    fn test_shorten_too_short() {

        // Create a line with length 2.
        let line: Line = Line {
            start: Point { x: 0.0, y: 0.0 },
            end: Point { x: 0.0, y: 2.0 },
        };

        // Shorten the line by 3.
        let shortened_line: Result<Line, LineError>  = line.shorten( 3.0 );
        match shortened_line {
            Ok( _ ) => {
                assert!( false, "It should not be possible to shorten the line more than it's lenght." );
            },
            Err( LineError::LineTooShortForShortening { current_length: _, requested_shortening: _ } ) => {
                assert!( true );
            }
        }
    }

    #[test]
    fn test_shorten_zero() {

        // Create a line with length 2.
        let line: Line = Line {
            start: Point { x: 0.0, y: 0.0 },
            end: Point { x: 0.0, y: 2.0 },
        };

        // Shorten the line by 1.
        let shortened_line: Result<Line, LineError>  = line.shorten( 0.0 );
        match shortened_line {
            Ok( shortened_line ) => {
                assert_eq!( shortened_line.length(), line.length() );
            },
            Err( _ ) => {
                assert!( false, "The line should be able to be shortened by zero." );
            }
        }
    }

    #[test]
    fn test_shorten_negative() {

        // Create a line with length 2.
        let line: Line = Line {
            start: Point { x: 0.0, y: 0.0 },
            end: Point { x: 0.0, y: 2.0 },
        };

        // Shorten the line by 1.
        let shortened_line: Result<Line, LineError>  = line.shorten( -1.0 );
        match shortened_line {
            Ok( shortened_line ) => {
                assert_eq!( shortened_line.length(), 3.0 );
            },
            Err( _ ) => {
                assert!( false, "The line should be able to be shortened by negative length." );
            }
        }
    }

    #[test]
    fn test_flip() {

        // Create a line.
        let line: Line = Line {
            start: Point { x: 1.0, y: 2.0 },
            end: Point { x: 3.0, y: 4.0 },
        };

        // Flip the line.
        let flipped_line: Line = line.flip();
        assert_eq!( flipped_line.start, line.end );
        assert_eq!( flipped_line.end, line.start );
    }

    #[test]
    fn test_flip_zero_length() {

        // Create a line.
        let line: Line = Line {
            start: Point { x: 0.0, y: 0.0 },
            end: Point { x: 0.0, y: 0.0 },
        };

        // Flip the line.
        let flipped_line: Line = line.flip();
        assert_eq!( flipped_line.start, line.end );
        assert_eq!( flipped_line.end, line.start );
    }

    #[test]
    fn test_intersect() {

        // Create two lines that intersect.
        let line1: Line = Line {
            start: Point { x: 1.0, y: 1.0 },
            end: Point { x: 5.0, y: 5.0 },
        };
        let line2: Line = Line {
            start: Point { x: 1.0, y: 5.0 },
            end: Point { x: 5.0, y: 1.0 },
        };

        // Check if the lines intersect.
        assert!( line1.intersects( &line2 ) );
    }

    #[test]
    fn test_not_intersect() {

        // Create two lines that do not intersect.
        let line1: Line = Line {
            start: Point { x: 1.0, y: 1.0 },
            end: Point { x: 5.0, y: 1.0 },
        };
        let line2: Line = Line {
            start: Point { x: 1.0, y: 2.0 },
            end: Point { x: 5.0, y: 5.0 },
        };

        // Check if the lines intersect.
        assert!( !line1.intersects( &line2 ) );
    }

    #[test]
    fn test_intersect_same_line() {

        // Create two lines that are the same.
        let line1: Line = Line {
            start: Point { x: 1.0, y: 1.0 },
            end: Point { x: 5.0, y: 1.0 },
        };
        let line2: Line = Line {
            start: Point { x: 1.0, y: 1.0 },
            end: Point { x: 5.0, y: 1.0 },
        };

        // Check if the lines intersect.
        assert!( line1.intersects( &line2 ) );
    }

    #[test]
    fn test_intersects_same_start() {

        // Create two lines that intersects on the starting point.
        let line1: Line = Line {
            start: Point { x: 1.0, y: 1.0 },
            end: Point { x: 5.0, y: 1.0 },
        };
        let line2: Line = Line {
            start: Point { x: 1.0, y: 1.0 },
            end: Point { x: 5.0, y: 5.0 },
        };

        // Check if the lines intersect.
        assert!( line1.intersects( &line2 ) );
    }

    #[test]
    fn test_intersects_same_end() {

        // Create two lines that intersects on the ending point.
        let line1: Line = Line {
            start: Point { x: 1.0, y: 1.0 },
            end: Point { x: 5.0, y: 5.0 },
        };
        let line2: Line = Line {
            start: Point { x: 4.0, y: 4.0 },
            end: Point { x: 5.0, y: 5.0 },
        };

        // Check if the lines intersect.
        assert!( line1.intersects( &line2 ) );
    }

}