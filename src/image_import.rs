extern crate image;

use crate::pixel::Rgba;
use crate::image::Image;

pub fn load_image(filepath: &str) -> Image<Rgba> {
    let img = image::open(filepath).unwrap();
    let buf = img.as_rgba8().unwrap(); // TODO: remove unwrap
    Image {
        width: buf.width() as usize,
        height: buf.height() as usize,
        vec: buf.pixels().map(|image::Rgba([r, g, b, a])| {
            Rgba(
                *r as f64 / std::u8::MAX as f64,
                *g as f64 / std::u8::MAX as f64,
                *b as f64 / std::u8::MAX as f64,
                *a as f64 / std::u8::MAX as f64)
        }).collect()
    }
}
