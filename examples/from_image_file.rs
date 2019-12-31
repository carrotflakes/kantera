extern crate kantera;
extern crate image;

use std::rc::Rc;
use kantera::pixel::Rgba;
use kantera::image::Image;
use kantera::render::{Range, Render, RenderOpt};
use kantera::export::render_to_mp4;
use kantera::renders::{
    image_render::{ImageRender, Sizing}
};

fn main() {
    let (width, height) = (320, 240);

    let img = Rc::new({
        let buf = image::open("./out.jpg").unwrap().to_rgba();
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
    });

    render_to_mp4(
        5.0, width, height, 30, 1,
        "from_image_file.mp4",
        &ImageRender {image: img.clone(), sizing: Sizing::Contain, default: Rgba(0.0, 0.0, 0.0, 0.0)});

    println!("done!");
}
