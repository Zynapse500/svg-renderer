use ::rectangle::Rectangle;

use super::point::Point;
use ::img::{Color, Image};


struct Node {
    center: Point,
    control_in: Point,
    control_out: Point,
}


pub struct Path {
    nodes: Vec<Node>,

    fill_color: Color,

    stroke_color: Color,
    stroke_width: f64,

    closed: bool,
}


impl Path {
    // Create a new path
    pub fn new() -> Path {
        Path {
            nodes: Vec::new(),

            fill_color: Color::rgb(0.95, 0.95, 0.95),

            stroke_color: Color::rgb(0.95, 0.05, 0.05),
            stroke_width: 25.0,

            closed: false,
        }
    }


    // Add a point to the path
    pub fn add_node(&mut self, point: Point, control_in: Point, control_out: Point) {
        self.nodes.push(Node {
            center: point,
            control_in,
            control_out,
        });
    }


    // Close the path, connecting the first and last point
    pub fn close(&mut self) {
        if !self.nodes.is_empty() {
            let first = self.nodes.first().unwrap().clone();
            self.nodes.push(first);
            self.closed = true;
        }
    }

    // Rasterize a path, returning a buffer of all the colors
    pub fn rasterize(&self, image_width: u32, image_height: u32, viewport: Rectangle) -> Image {
        let mut image = Image::with_size(image_width, image_height);

        // Construct scanlines
        let dx = (viewport.right - viewport.left) / image_width as f64;
        let dy = (viewport.bottom - viewport.top) / image_height as f64;
        for row in 0..image_height {
            // let x = viewport.bottom + col as f64 * dy;
            let y = viewport.top + row as f64 * dy;

            let mut intersections = Vec::new();
            //
            // (1-t)^3*P = ((1-t)(1-t)(1-t))*P = (1 - 3*t + 3t*t - t*t*t) * P
            // 3*(1-t)^2*t*P = 3*((1-t)(1-t))*t*P = 3*(1 - 2*t + t*t)*t * P = (3t - 6*t*t + 3*t*t*t) * P
            // 3*(1-t)*t*t*P = (3*t*t - 3*t*t*t) * P
            // t*t*t * P

            // (1 - 3*t + 3t*t - t*t*t) * Pa + (3t - 6*t*t + 3*t*t*t) * Pb + (3*t*t - 3*t*t*t) * Pc + t*t*t * Pd
            // Pa - 3*Pa*t + 3*Pa*t*t - Pa*t*t*t + 3*Pb*t - 6*Pb*t*t + 3*Pb*t*t*t + 3*Pc*t*t - 3*Pc*t*t*t + Pd*t*t*t
            // Pa + (-3Pa+3Pb)*t + (3Pa-6Pb+3Pc)*t*t + (-Pa+3Pb-3Pc+Pd)*t*t*t

            // At^3 + Bt^2 + Ct + D =>

            // A = - Pa +3Pb -3Pc +Pd
            // B =  3Pa -6Pb +3Pc
            // C = -3Pa +3Pb
            // D =   Pa

            for i in 0..self.nodes.len() - 1 {
                let node_start: Node = self.nodes[i];
                let node_end: Node = self.nodes[i + 1];

                let p_a = node_start.center.y;
                let p_b = node_start.control_out.y;
                let p_c = node_end.control_in.y;
                let p_d = node_end.center.y;

                let a: f64 = -p_a + 3.0 * p_b - 3.0 * p_c + p_d;
                let b: f64 = 3.0 * p_a - 6.0 * p_b + 3.0 * p_c;
                let c: f64 = -3.0 * p_a + 3.0 * p_b;
                let d: f64 = p_a;

                let (t0, t1, t2) = solve_cubic(A.y, B.y, C.y, D.y, y);
            }
        }
    }
}


// Solve a cubic equation:
// Ax^3 + Bx^2 + Cx + D = y
fn solve_cubic(a: f64, b: f64, c: f64, d: f64, y: f64) -> (Option<f64>, Option<f64>, Option<f64>) {
    let d_0 = b*b - 3.0*a*c;
    let d_1 = 2.0*b*b*b - 9.0*a*b*c + 27.0*a*a*d;
    let discriminant = (d_1*d_1 - 4.0*d_0*d_0*d_0) / (-27.0*a*a);
    let 
}