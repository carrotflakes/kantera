extern crate kantera;

use std::rc::Rc;
use kantera::pixel::Rgba;
use kantera::render::Render;
use kantera::export::render_to_mp4;
use kantera::ffmpeg::import_image;
use kantera::renders::image_render::{ImageRender, Sizing};

fn main() {
    let (width, height) = (320, 240);

    let image = Rc::new(import_image("./out.jpg"));

    render_to_mp4(
        5.0, width, height, 30, 1,
        "ffmpeg_import_image.mp4",
        &ImageRender {
            image: image.clone(), sizing: Sizing::Fit, default: Rgba(0.0, 0.0, 0.0, 0.0),
            interpolation: kantera::interpolation::Bilinear
    });

    println!("done!");
}
