use rand;

// Utility Functions

pub fn random_double(min: f64, max: f64) -> f64 {
    // Returns a random real in [min,max).
    min + (max - min) * rand::random::<f64>()
}