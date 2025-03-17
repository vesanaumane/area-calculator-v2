pub trait AreaCalculatable {
    
    // Returns the area of the shape, or -1.0 if the shape is not valid.
    fn area(&self) -> f64;
}