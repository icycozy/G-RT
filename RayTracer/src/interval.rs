use std::ops::Add;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Interval {
    pub min: f64,
    pub max: f64,
}

impl Interval {
    pub fn new() -> Self {
        Interval {
            min: f64::INFINITY,
            max: f64::NEG_INFINITY,
        }
    }

    pub fn with_values(min: f64, max: f64) -> Self {
        Interval { min, max }
    }

    pub fn from_intervals(a: Interval, b: Interval) -> Interval {
        // Create the interval tightly enclosing the two input intervals.
        let min = if a.min <= b.min { a.min } else { b.min };
        let max = if a.max >= b.max { a.max } else { b.max };
        Interval { min, max }
    }

    pub fn size(&self) -> f64 {
        self.max - self.min
    }

    pub fn contains(&self, x: f64) -> bool {
        self.min <= x && x <= self.max
    }

    pub fn surrounds(&self, x: f64) -> bool {
        self.min < x && x < self.max
    }

    pub fn clamp(&self, x: f64) -> f64 {
        if x < self.min {
            self.min
        } else if x > self.max {
            self.max
        } else {
            x
        }
    }

    pub fn expand(&self, delta: f64) -> Interval {
        let padding = delta / 2.0;
        Interval::with_values(self.min - padding, self.max + padding)
    }
}

impl Add<f64> for Interval {
    type Output = Interval;

    fn add(self, displacement: f64) -> Interval {
        Interval::with_values(self.min + displacement, self.max + displacement)
    }
}

impl Add<Interval> for f64 {
    type Output = Interval;

    fn add(self, ival: Interval) -> Interval {
        ival + self
    }
}

pub const UNIVERSE: Interval = Interval {
    min: f64::NEG_INFINITY,
    max: f64::INFINITY,
};