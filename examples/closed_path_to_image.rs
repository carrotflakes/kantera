extern crate kantera;

use std::rc::Rc;
use kantera::export::render_to_mp4;
use kantera::renders::image_render::{ImageRender, Sizing};
use kantera::path_to_image::closed_path_to_image;
use kantera::pixel::Rgba;
use kantera::v::Vec2;
use kantera::path::{Path, Point};

fn main() {
    let path = Path::new(Vec2(1.0, 1.0))
        .append(1.0, Vec2(90.0, 50.0), Point::Linear)
        .append(1.0, Vec2(1.0, 90.0), Point::Bezier(Vec2(0.0, 20.0), Vec2(0.0, -30.0)));
    let image = Rc::new(closed_path_to_image((-10, -10, 100, 100), Rgba(1.0, 0.0, 0.0, 1.0), Rgba(0.1, 0.7, 0.0, 1.0), 3.0, &path));
    render_to_mp4(
        5.0,
        320,
        240,
        30,
        1,
        "closed_path_to_image.mp4",
        &ImageRender {
            image: image.clone(), sizing: Sizing::Contain, default: Rgba(0.0, 0.0, 0.0, 0.0),
            interpolation: kantera::interpolation::Bilinear
        });

    println!("done!");
}
