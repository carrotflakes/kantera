extern crate kantera;

use std::rc::Rc;
use kantera::pixel::Rgba;
use kantera::render::{Range, Render, RenderOpt};
use kantera::export::render_to_mp4;
use kantera::renders::{
    sequence::Sequence,
    image_render::{ImageRender, Sizing}
};
use kantera::util::hsl_to_rgb;

fn main() {
    let (width, height) = (320, 240);

    let image = Rc::new(kantera::cairo::render_image(16, 16, &|ctx| {
        ctx.set_source_rgb(0.9, 0.9, 0.9);
        ctx.paint();

        ctx.move_to(0.0, 0.0);
        ctx.line_to(16.0, 16.0);
        ctx.set_source_rgb(1.0, 0.0, 0.0);
        ctx.stroke();
        ctx.move_to(16.0, 0.0);
        ctx.line_to(0.0, 16.0);
        ctx.set_source_rgb(1.0, 0.0, 0.0);
        ctx.stroke();
    }));

    render_to_mp4(
        10.0, width, height, 30, 1,
        "image_render.mp4",
        &Sequence::<Rgba, Box<dyn Render<Rgba>>>::new()
            .append(
                0.0,
                true,
                Box::new(ImageRender {image: image.clone(), sizing: Sizing::Fit, default: Rgba(0.0, 0.0, 0.0, 0.0)}))
            .append(
                2.0,
                true,
                Box::new(ImageRender {image: image.clone(), sizing: Sizing::Contain, default: Rgba(0.0, 0.0, 0.0, 0.0)}))
            .append(
                4.0,
                true,
                Box::new(ImageRender {image: image.clone(), sizing: Sizing::Cover, default: Rgba(0.0, 0.0, 0.0, 0.0)}))
            .append(
                6.0,
                true,
                Box::new(ImageRender {image: image.clone(), sizing: Sizing::DotByDot, default: Rgba(0.0, 0.0, 0.0, 0.0)}))
            );

    println!("done!");
}
