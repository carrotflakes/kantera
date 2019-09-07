extern crate kantera;

use std::rc::Rc;
use kantera::pixel::Rgba;
use kantera::render::{Range, Render, RenderOpt};
use kantera::export::render_to_mp4;
use kantera::renders::{
    sequence::Sequence,
    image_render::{ImageRender, Sizing},
    transform::{Transform, Mat}
};
use kantera::path::{Path, PointType};
use kantera::util::hsl_to_rgb;

fn main() {
    let (width, height) = (320, 240);

    let image = Rc::new(kantera::cairo::render_image(width, height, &|ctx| {
        ctx.set_source_rgb(0.9, 0.9, 0.9);
        ctx.paint();

        ctx.move_to(0.0, 0.0);
        ctx.line_to(width as f64, height as f64);
        ctx.set_source_rgb(1.0, 0.0, 0.0);
        ctx.stroke();
        ctx.move_to(width as f64, 0.0);
        ctx.line_to(0.0, height as f64);
        ctx.set_source_rgb(1.0, 0.0, 0.0);
        ctx.stroke();
    }));

    render_to_mp4(
        10.0, width, height, 30, 1,
        "transform.mp4",
        &Sequence::new()
            .append(
                0.0,
                true,
                Box::new(Transform {
                    render: Box::new(ImageRender {image: image.clone(), sizing: Sizing::Fit}),
                    transformer: Box::new(Mat::new()
                                          .get_transformer())
                }))
            .append(
                2.0,
                true,
                Box::new(Transform {
                    render: Box::new(ImageRender {image: image.clone(), sizing: Sizing::Fit}),
                    transformer: Box::new(Mat::new()
                                          .scale(0.5, 0.5)
                                          .get_transformer())
                }))
            .append(
                4.0,
                true,
                Box::new(Transform {
                    render: Box::new(ImageRender {image: image.clone(), sizing: Sizing::Fit}),
                    transformer: Box::new(Mat::new()
                                          .scale(0.5, 0.5)
                                          .translate(160.0, 120.0)
                                          .get_transformer())
                }))
            .append(
                6.0,
                true,
                Box::new(Transform {
                    render: Box::new(ImageRender {image: image.clone(), sizing: Sizing::Fit}),
                    transformer: Box::new(Mat::new()
                                          .scale(0.5, 0.5)
                                          .translate(160.0, 120.0)
                                          .rotate(20.0f64.to_radians())
                                          .get_transformer())
                }))
            .append(
                8.0,
                true,
                Box::new(Transform {
                    render: Box::new(ImageRender {image: image.clone(), sizing: Sizing::Fit}),
                    transformer: Box::new(Mat::new()
                                          .translate(-160.0, -120.0)
                                          .rotate(20.0f64.to_radians())
                                          .translate(160.0, 120.0)
                                          .get_transformer())
                }))
            );

    println!("done!");
}
