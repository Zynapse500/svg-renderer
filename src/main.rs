
extern crate image;

use std::path::Path;

#[derive(Clone)]
struct Color {
    r: u8,
    g: u8,
    b: u8,
    a: u8
}

impl Color {
    pub fn rgb(r: u8, g: u8, b: u8) -> Color {
        Color { r, g, b, a: 255 }
    }

    pub fn rgba(r: u8, g: u8, b: u8, a: u8) -> Color {
        Color { r, g, b, a }
    }
}

struct Image {
    width: u32,
    height: u32,

    buffer: Vec<Color>
}

impl Image {
    pub fn with_size(width: u32, height: u32) -> Image {
        Image {
            width, height,
            buffer: vec![Color::rgb(0, 0, 0); (width * height) as usize]
        }
    }

    pub fn set_pixel(&mut self, x: u32, y: u32, color: Color) {
        if let Some(pixel) = self.get_pixel_mut(x, y) {
            *pixel = color;
        }
    } 

    pub fn get_pixel_mut<'a>(&'a mut self, x: u32, y: u32) -> Option<&'a mut Color> {
        self.buffer.get_mut((x + y * self.width) as usize)
    }


    pub fn save(&self, path: &str) -> std::io::Result<()> 
     {
        let mut buffer = Vec::with_capacity((self.width * self.height * 4) as usize);
        
        for color in self.buffer.iter() {
            buffer.push(color.r);
            buffer.push(color.g);
            buffer.push(color.b);
            buffer.push(color.a);
        }

        // Create directory if it does not exist
        let mut buf = std::path::PathBuf::from(path);
        if !buf.exists() {
            println!("Not exists!");
            buf.pop();

            if let Err(e) = std::fs::create_dir_all(buf) {
                return Err(e);
            }
        }

        // Save image
        image::save_buffer(path, buffer.as_slice(), self.width, self.height, image::RGBA(8))
    }
}



enum Shape {
    Rectangle{left: f64, right: f64, top: f64, bottom: f64}
}

struct Renderer {
    shapes: Vec<Shape>
}

impl Renderer {
    pub fn new() -> Renderer {
        Renderer {
            shapes: Vec::new()
        }
    }


    pub fn fill_rectangle(&mut self, x: f64, y: f64, w: f64, h: f64) {
        self.shapes.push(Shape::Rectangle{
            left: x,
            top: y,
            right: x + w,
            bottom: y + h
        });
    }


    pub fn rasterize(&self, width: u32, height: u32) -> Image {
        let mut image = Image::with_size(width, height);

        for x in 0..width {
            for y in 0..height {
                image.set_pixel(x, y, Color::rgb(x as u8, y as u8, 0))
            }
        }

        for shape in self.shapes.iter() {
            match shape {
                &Shape::Rectangle {left, right, top, bottom} => {
                    for x in left.round() as u32..right.round() as u32 {
                        for y in top.round() as u32..bottom.round() as u32 {
                            image.set_pixel(x, y, Color::rgb(255, 0, 0));
                        }
                    }
                }
            }
        }

        image
    }
}



fn main() {
    let mut renderer = Renderer::new();

    renderer.fill_rectangle(64.0, 64.0, 64.0, 128.0);

    let image = renderer.rasterize(256, 256);
    if let Err(e) = image.save("out/out.png") {
        println!("{}", e);
    }
}
