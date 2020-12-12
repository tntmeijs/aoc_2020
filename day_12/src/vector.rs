use std::ops;
use std::f64;

pub struct Vector2D {
    x: f64,
    y: f64
}

impl Vector2D {
    // Construct a new vector with all values set to zero
    pub fn new() -> Self {
        Vector2D { x: 0.0, y: 0.0 }
    }

    // Calculate the Manhattan distance between two points
    pub fn manhattan_distance(a: &Self, b: &Self) -> f64 {
        let abs_horizontal = (a.x - b.y).abs();
        let abs_vertical = (a.y - b.y).abs();
        abs_horizontal + abs_vertical
    }

    // Rotate this vector by N degrees
    // Reference: https://matthew-brett.github.io/teaching/rotation_2d.html
    pub fn rotate(&mut self, degrees: i32) {
        let radians = degrees as f64 * f64::consts::PI / 180.0;

        let cos = f64::cos(radians);
        let sin = f64::sin(radians);

        let x = self.x * cos - self.y * sin;
        let y = self.x * sin + self.y * cos;

        self.x = x;
        self.y = y;
    }
}

impl ops::Add for Vector2D {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Vector2D { x: self.x + rhs.x, y: self.y + rhs.y }
    }
}

impl ops::Sub for Vector2D {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Vector2D { x: self.x - rhs.x, y: self.y - rhs.y }
    }
}
