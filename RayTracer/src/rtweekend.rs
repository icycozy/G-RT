use rand;

// Utility Functions

pub fn random_double(min: f64, max: f64) -> f64 {
    // Returns a random real in [min,max).
    min + (max - min) * rand::random::<f64>()
}

pub fn random_int(min: i32, max: i32) -> i32 {
    // Returns a random integer in [min,max].
    random_double(min as f64, (max + 1) as f64) as i32}