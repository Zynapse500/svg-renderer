
use std::ops::{Add, Sub, Mul};

#[derive(Copy, Clone, PartialEq)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    pub fn new(x: f64, y: f64) -> Point {
        Point { x, y }
    }


    // The distance from the origin
    pub fn length(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }


    // The distance between two points
    pub fn distance(&self, other: Point) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;

        (dx * dx + dy * dy).sqrt()
    }


    // The normalized form
    pub fn normal(&self) -> Point {
        let len = (self.x * self.x + self.y * self.y).sqrt();
        Point {
            x: self.x / len,
            y: self.y / len
        }
    }


    // The dot product
    pub fn dot(&self, rhs: Point) -> f64 {
        self.x * rhs.x + self.y * rhs.y
    }
}


impl Add<Point> for Point {
    type Output = Point;

    fn add(mut self, rhs: Point) -> Self::Output {
        self.x += rhs.x;
        self.y += rhs.y;

        self
    }
}

impl Sub<Point> for Point {
    type Output = Point;

    fn sub(mut self, rhs: Point) -> Self::Output {
        self.x -= rhs.x;
        self.y -= rhs.y;

        self
    }
}




impl Mul<f64> for Point {
    type Output = Point;

    fn mul(mut self, rhs: f64) -> Self::Output {
        self.x *= rhs;
        self.y *= rhs;

        self
    }
}


impl Mul<Point> for f64 {
    type Output = Point;

    fn mul(self, mut rhs: Point) -> Self::Output {
        rhs.x *= self;
        rhs.y *= self;

        rhs
    }
}
