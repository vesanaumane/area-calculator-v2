pub trait AreaCalculatable {
    
    // Returns the area of the shape, or zero if the shape is not valid.
    fn area(&self) -> f64;
}