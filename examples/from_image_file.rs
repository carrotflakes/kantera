extern crate kantera;
extern crate image;

use std::rc::Rc;
use kantera::pixel::Rgba;
use kantera::image::Image;
use kantera::export::render_to_mp4;
use kantera::renders::{
    image_render::{ImageRender, Sizing}
};

fn main() {
    let (width, height) = (320, 240);

    let img = Rc::new({
        let img = image::open("./out.jpg").unwrap();
        let buf = img.as_rgb8().unwrap();
        Image {
            width: buf.width() as usize,
            height: buf.height() as usize,
            vec: buf.pixels().map(|image::Rgb([r, g, b])| {
                Rgba(
                    *r as f64 / std::u8::MAX as f64,
                    *g as f64 / std::u8::MAX as f64,
                    *b as f64 / std::u8::MAX as f64,
                    1.0)
            }).collect()
        }
    });

    render_to_mp4(
        5.0, width, height, 30, 1,
        "from_image_file.mp4",
        &ImageRender {
            image: img.clone(),
            sizing: Sizing::Contain,
            default: Rgba(0.0, 0.0, 0.0, 0.0),
            interpolation: kantera::interpolation::Bilinear
        });

    println!("done!");
}
