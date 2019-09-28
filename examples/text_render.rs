extern crate kantera;

use std::rc::Rc;
use kantera::{
    pixel::Rgba,
    export::render_to_mp4,
    renders::{
        plain::Plain,
        composite::{Composite, CompositeMode},
        image_render::{ImageRender, Sizing}
    },
    path::Path,
    text::{Font, render}
};

fn main() {
    let font_path = "/usr/share/fonts/truetype/dejavu/DejaVuSansMono.ttf";
    let bytes = std::fs::read(font_path).unwrap();
    let font = Font::from_bytes(&bytes).unwrap();
    let (width, height) = (320, 240);

    let image = Rc::new(render(&font, "nyahaha"));

    render_to_mp4(
        10.0, width, height, 30, 1,
        "text_render.mp4",
        &Composite {
            layers: vec![
                (
                    Box::new(Plain(Rgba(0.1, 0.1, 0.1, 1.0))),
                    CompositeMode::None
                ),
                (
                    Box::new(ImageRender {image: image.clone(), sizing: Sizing::Contain}),
                    CompositeMode::Normal(Path::new(1.0))
                )
            ]
        });

    println!("done!");
}
