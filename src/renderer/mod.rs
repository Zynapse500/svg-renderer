mod path;
mod point;

use ::rectangle::Rectangle;

use self::path::Path;
use self::point::Point;

use ::img::{Image, Color};

pub struct Renderer {
    paths: Vec<Path>
}

impl Renderer {
    pub fn new() -> Renderer {
        Renderer {
            paths: Vec::new()
        }
    }


    pub fn fill_rectangle(&mut self, x: f64, y: f64, w: f64, h: f64) {
        self.paths.push({
            let mut path = Path::new();
            path.add_point(Point::new(x, y));
            path.add_point(Point::new(x + w, y));
            path.add_point(Point::new(x + w, y + h));
            path.add_point(Point::new(x, y + h));
            path.close();
            path
        });
    }


    pub fn rasterize(&self, width: u32, height: u32, viewport: Rectangle) -> Image {
        let mut image = Image::with_size(width, height);

        // Draw temporary background
        for x in 0..width {
            for y in 0..height {
                let color = Color {
                    r: x as f64 / width as f64,
                    g: y as f64 / height as f64,
                    b: 0.0,
                    a: 1.0,
                };

                image.set_pixel(x, y, color)
            }
        }

        // Render paths
        let samples = 4;
        let colormap_width = width * samples;
        let colormap_height = height * samples;

        for path in self.paths.iter() {

            let colormap = path.rasterize(colormap_width, colormap_height, viewport);

            for x in 0..width {
                for y in 0..height {
                    let mut average = Color::rgba(0.0, 0.0, 0.0, 0.0);
                    let mut color_samples = 0.0;

                    for sx in 0..samples {
                        for sy in 0..samples {
                            let point = Point::new(
                                x as f64 + sx as f64 / samples as f64,
                                y as f64 + sy as f64 / samples as f64,
                            );

                            if let Some(color) = path.get_color(point) {
                                average.r += color.r;
                                average.g += color.g;
                                average.b += color.b;
                                average.a += color.a;
                                color_samples += color.a;
                            }
                        }
                    }

                    average.r /= (color_samples) as f64;
                    average.g /= (color_samples) as f64;
                    average.b /= (color_samples) as f64;
                    average.a /= (samples * samples) as f64;

                    image.set_pixel(x, y, average)
                }
            }
        }

        image
    }
}