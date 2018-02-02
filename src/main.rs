extern crate image;

mod rectangle;
use rectangle::Rectangle;

mod img;
use img::{Image, Color};

mod renderer;
use renderer::Renderer;


fn main() {
    let mut renderer = Renderer::new();

    renderer.fill_rectangle(32.0, 32.0, 64.0, 96.0);

    let viewport = Rectangle::new(0.0, 256.0, 0.0, 256.0);
    let image = renderer.rasterize(256, 256, viewport);
    if let Err(e) = image.save("out/out.png") {
        println!("{}", e);
    }
}
