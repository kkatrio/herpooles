use std::ops::{Add, Mul};

#[derive(Copy, Clone)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

// Not used
impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Add<Vector> for Point {
    type Output = Self;

    fn add(self, v: Vector) -> Self {
        Point {
            x: self.x + v.x,
            y: self.y + v.y,
        }
    }
}

#[derive(Copy, Clone)]
pub struct Vector {
    pub x: f32,
    pub y: f32,
}

impl Vector {
    pub fn new(a: Point, b: Point) -> Vector {
        Vector {
            x: b.x - a.x,
            y: b.y - a.y,
        }
    }

    pub fn unit_vec(&self) -> Vector {
        let magnitude = (self.x * self.x + self.y * self.y).sqrt();
        Vector {
            x: self.x / magnitude,
            y: self.y / magnitude,
        }
    }
}

impl Mul<f32> for Vector {
    type Output = Self;

    fn mul(self, scalar: f32) -> Self {
        Vector {
            x: scalar * self.x,
            y: scalar * self.y,
        }
    }
}
