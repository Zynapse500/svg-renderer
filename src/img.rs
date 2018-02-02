use std;
use image;


#[derive(Clone)]
pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64,
    pub a: f64,
}

impl Color {
    pub fn rgb(r: f64, g: f64, b: f64) -> Color {
        Color { r, g, b, a: 1.0 }
    }

    pub fn rgba(r: f64, g: f64, b: f64, a: f64) -> Color {
        Color { r, g, b, a }
    }
}



pub struct Image {
    width: u32,
    height: u32,

    buffer: Vec<Color>,
}

impl Image {
    // Create an image with predefined size
    pub fn with_size(width: u32, height: u32) -> Image {
        Image {
            width,
            height,
            buffer: vec![Color::rgb(0.0, 0.0, 0.0); (width * height) as usize],
        }
    }


    // Set the color of a pixel
    pub fn set_pixel(&mut self, x: u32, y: u32, color: Color) {
        if let Some(pixel) = self.get_pixel_mut(x, y) {
            *pixel = color;
        }
    }


    // Get the color of a pixel
    pub fn get_pixel<'a>(&'a self, x: u32, y: u32) -> Option<&'a Color> {
        self.buffer.get((x + y * self.width) as usize)
    }

    // Get the color of a pixel
    pub fn get_pixel_mut<'a>(&'a mut self, x: u32, y: u32) -> Option<&'a mut Color> {
        self.buffer.get_mut((x + y * self.width) as usize)
    }


    // Save image to path
    pub fn save(&self, path: &str) -> std::io::Result<()>
    {
        let mut buffer = Vec::with_capacity((self.width * self.height * 4) as usize);

        for color in self.buffer.iter() {
            buffer.push((255.0 * color.r) as u8);
            buffer.push((255.0 * color.g) as u8);
            buffer.push((255.0 * color.b) as u8);
            buffer.push((255.0 * color.a) as u8);
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

        // Save img
        image::save_buffer(path, buffer.as_slice(), self.width, self.height, image::RGBA(8))
    }
}
