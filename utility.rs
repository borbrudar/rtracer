//use std::f64::INFINITY;

pub const INFINITY : f64 = f64::INFINITY;

pub fn degrees_to_radians(degrees : f64) -> f64{
    degrees * std::f64::consts::PI / 180.0
}
