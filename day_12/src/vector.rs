use std::ops;
use std::f64;

#[derive(Clone, Copy)]
pub struct Vector2D {
    pub x: i32,
    pub y: i32
}

impl Vector2D {
    // Construct a new vector with all values set to zero
    pub fn new() -> Self {
        Vector2D { x: 0, y: 0 }
    }

    // Calculate the Manhattan distance between two points
    pub fn manhattan_distance(a: &Self, b: &Self) -> u32 {
        let abs_horizontal = (a.x - b.x).abs();
        let abs_vertical = (a.y - b.y).abs();
        (abs_horizontal + abs_vertical) as u32
    }

    // Rotate this vector by N degrees
    // Reference: https://matthew-brett.github.io/teaching/rotation_2d.html
    pub fn rotate(&mut self, degrees: i32) {
        let radians = degrees as f64 * f64::consts::PI / 180.0;

        let cos = f64::cos(radians);
        let sin = f64::sin(radians);

        let f64x = self.x as f64;
        let f64y = self.y  as f64;

        let x = f64x * cos - f64y * sin;
        let y = f64x * sin + f64y * cos;

        self.x = x as i32;
        self.y = y as i32;
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

impl ops::Mul for Vector2D {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Vector2D { x: self.x * rhs.x, y: self.y * rhs.y }
    }
}
