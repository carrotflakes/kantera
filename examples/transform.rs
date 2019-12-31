extern crate kantera;

use std::rc::Rc;
use kantera::pixel::Rgba;
use kantera::render::Render;
use kantera::export::render_to_mp4;
use kantera::renders::{
    sequence::Sequence,
    image_render::{ImageRender, Sizing},
    transform::{Transform, Mat, path_to_transformer}
};
use kantera::path::{Path, Point};
use kantera::v::Vec2;

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
        &Sequence::<Rgba, Box<dyn Render<Rgba>>>::new()
            .append(
                0.0,
                true,
                Box::new(Transform::<Rgba, Box<dyn Render<Rgba>>>::new(
                    Box::new(ImageRender {image: image.clone(), sizing: Sizing::Fit, default: Rgba(0.0, 0.0, 0.0, 0.0)}),
                    Box::new(Mat::new().get_transformer())
                )))
            .append(
                1.0,
                true,
                Box::new(Transform::<Rgba, Box<dyn Render<Rgba>>>::new(
                    Box::new(ImageRender {image: image.clone(), sizing: Sizing::Fit, default: Rgba(0.0, 0.0, 0.0, 0.0)}),
                    Box::new(Mat::new()
                        .scale(0.5, 0.5)
                        .get_transformer())
                )))
            .append(
                2.0,
                true,
                Box::new(Transform::<Rgba, Box<dyn Render<Rgba>>>::new(
                    Box::new(ImageRender {image: image.clone(), sizing: Sizing::Fit, default: Rgba(0.0, 0.0, 0.0, 0.0)}),
                    Box::new(Mat::new()
                        .scale(0.5, 0.5)
                        .translate(160.0, 120.0)
                        .get_transformer())
                )))
            .append(
                3.0,
                true,
                Box::new(Transform::<Rgba, Box<dyn Render<Rgba>>>::new(
                    Box::new(ImageRender {image: image.clone(), sizing: Sizing::Fit, default: Rgba(0.0, 0.0, 0.0, 0.0)}),
                    Box::new(Mat::new()
                        .scale(0.5, 0.5)
                        .translate(160.0, 120.0)
                        .rotate(20.0f64.to_radians())
                        .get_transformer())
                )))
            .append(
                4.0,
                true,
                Box::new(Transform::<Rgba, Box<dyn Render<Rgba>>>::new(
                    Box::new(ImageRender {image: image.clone(), sizing: Sizing::Fit, default: Rgba(0.0, 0.0, 0.0, 0.0)}),
                    Box::new(Mat::new()
                        .translate(-160.0, -120.0)
                        .rotate(20.0f64.to_radians())
                        .translate(160.0, 120.0)
                        .get_transformer())
                )))
            .append(
                5.0,
                true,
                Box::new(Transform::<Rgba, Box<dyn Render<Rgba>>>::new(
                    Box::new(ImageRender {image: image.clone(), sizing: Sizing::Fit, default: Rgba(0.0, 0.0, 0.0, 0.0)}),
                    path_to_transformer(
                        Path::new(Vec2(0.0, 0.0))
                        .append(0.5, Vec2(0.0, 0.0), Point::Constant)
                        .append(0.5, Vec2(0.25, 0.0), Point::Linear)
                        .append(1.0, Vec2(-0.25, 0.0), Point::Linear),
                        Path::new(Vec2(1.0, 1.0))
                        .append(0.5, Vec2(0.25, 0.25), Point::Linear)
                        .append(1.0, Vec2(0.25, 0.25), Point::Linear)
                        .append(0.5, Vec2(0.5, 0.5), Point::Linear),
                        Path::new(0.0)
                        .append(1.0, 0.0, Point::Constant)
                        .append(1.0, std::f64::consts::PI, Point::Linear)
                        .append(1.0, 0.0, Point::Linear)
                    )
                )))
            );

    println!("done!");
}
