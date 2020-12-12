use std::ops;
use std::f64;

pub struct Vector {
    x: f64,
    y: f64
}

impl Vector {
    // Construct a new vector with all values set to zero
    pub fn new() -> Self {
        Self { x: 0.0, y: 0.0 }
    }

    // Calculate the Manhattan distance between two points
    pub fn manhattan_distance(a: &Self, b: &Self) -> f64 {
        let abs_horizontal = (a.x - b.y).abs();
        let abs_vertical = (a.y - b.y).abs();
        abs_horizontal + abs_vertical
    }

    // Rotate this vector by N degrees
    // Reference: https://matthew-brett.github.io/teaching/rotation_2d.html
    fn rotate(&mut self, degrees: i32) {
        let radians = degrees as f64 * f64::consts::PI / 180.0;
        self.x = f64::cos(radians) - f64::sin(radians);
        self.y = f64::sin(radians) + f64::cos(radians);
    }
}

impl ops::Add for Vector {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self { x: self.x + rhs.x, y: self.y + rhs.y }
    }
}

impl ops::Sub for Vector {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self { x: self.x - rhs.x, y: self.y - rhs.y }
    }
}
