extern crate kantera;

use std::rc::Rc;
use kantera::{
    pixel::Rgba,
    export::render_to_mp4,
    render::Render,
    renders::{
        plain::Plain,
        composite::{Composite, CompositeMode},
        image_render::{ImageRender, Sizing}
    },
    path::Path,
    text::{Font, render}
};

fn main() {
    let font_path = "./IPAexfont00401/ipaexg.ttf";
    let bytes = std::fs::read(font_path).unwrap();
    let font = Font::from_bytes(&bytes).unwrap();
    let (width, height) = (320, 240);

    let image = Rc::new(render(&font, 32.0, "にゃはは").map(|v| Rgba(0.1, 0.1, 0.1, *v)));

    render_to_mp4(
        10.0, width, height, 30, 1,
        "text_render.mp4",
        &Composite::<Box<dyn Render<Rgba>>> {
            layers: vec![
                (
                    Box::new(Plain::new(Rgba(1.0, 1.0, 1.0, 1.0))),
                    CompositeMode::None
                ),
                (
                    Box::new(ImageRender {
                        image: image.clone(), sizing: Sizing::Contain, default: Rgba(0.0, 0.0, 0.0, 0.0),
                        interpolation: kantera::interpolation::Bilinear
                    }),
                    CompositeMode::Normal(Path::new(1.0))
                )
            ]
        });

    println!("done!");
}
