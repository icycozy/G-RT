use std::f64::INFINITY;
use std::f64::consts::PI;
use rand;


// Utility Functions

fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}


pub fn random_double(min: f64, max: f64) -> f64 {
    // Returns a random real in [min,max).
    min + (max - min) * rand::random::<f64>()
}