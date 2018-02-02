
use ::rectangle::Rectangle;

use super::point::Point;
use ::img::{Color, Image};

pub struct Path {
    points: Vec<Point>,

    fill_color: Color,

    stroke_color: Color,
    stroke_width: f64
}


impl Path {

    // Create a new path
    pub fn new() -> Path {
        Path {
            points: Vec::new(),

            fill_color: Color::rgb(0.95, 0.95, 0.95),

            stroke_color: Color::rgb(0.95, 0.05, 0.05),
            stroke_width: 25.0
        }
    }


    // Add a point to the path
    pub fn add_point(&mut self, point: Point) {
        self.points.push(point);
    }


    // Close the path, connecting the first and last point
    pub fn close(&mut self) {
        if !self.points.is_empty() {
            let first = self.points.first().unwrap().clone();
            self.points.push(first);
        }
    }


    // Get the color of a point if the point is on the path
    pub fn get_color(&self, point: Point) -> Option<Color> {
        // 1. Check if we are on the stroke
        let width = self.stroke_width / 2.0;

        for i in 0..self.points.len()-1 {
            // The current line
            let a: Point = self.points[i];
            let b: Point = self.points[i + 1];

            let delta = b - a;
            let length = delta.length();


            // Project the point onto the line
            let direction = delta.normal();
            let projection = direction.dot(point - a);

            // Check if point is perpendicular to line
            if projection < 0.0 || projection > length {
                // If we are past the line, check for nodes
                // If we are inside a node, add a rounded corner
                if point.distance(a) < width || point.distance(b) < width {
                    return Some(self.stroke_color.clone());
                } else {
                    continue;
                }
            }

            // Find the distance to the closest point
            let closest = a + direction * projection;
            let distance = closest.distance(point);

            // Check if the point is on the stroke and return stroke color
            if distance < width {
                return Some(self.stroke_color.clone());
            }
        }

        // 2. If the path is closed, check if the point is inside the path and return the fill color
        if self.points.len() > 2 {
            if self.points.first().unwrap() == self.points.last().unwrap() {
                // The path is closed, are we inside it?

            }
        }

        None
    }


    // Rasterize a path, returning a buffer of all the colors
    pub fn rasterize(&self, image_width: u32, image_height: u32, viewport: Rectangle) -> Image {
        let mut image = Image::with_size(image_width, image_height);

        // Construct scanlines
        for row in 0..image_height {

        }
    }
}
