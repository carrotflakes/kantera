extern crate kantera;
extern crate image;

use std::rc::Rc;
use kantera::{
    pixel::Rgba,
    image::Image,
    render::{Range, Render, RenderOpt},
    export::render_to_mp4,
    renders::{
        image_render::{ImageRender, Sizing},
        frame::{Frame, FrameType},
        rgb_transform::RgbTransform
    },
    util::u32_noise
};

fn fractal_noise(n: usize, x: u32, y: u32, z: u32) -> f64 {
    let mut v = 0.0;
    for i in 0..n {
        v += u32_noise(x >> i, y >> i, z) as f64;
    }
    v / std::u32::MAX as f64 / n as f64
}

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
        "glitch00.mp4",
        &RgbTransform {
            render: Box::new(Frame {
                render: Box::new(ImageRender {
                    image: img.clone(),
                    sizing: Sizing::Contain
                }),
                frame_type: FrameType::Repeat
            }),
            transformer: Box::new(|u, v, time, (w, h)| {
                let x = (u / 20.0 * w as f64).floor() as u32;
                let y = (v / 5.0 * h as f64).floor() as u32;
                let mut d = fractal_noise(5, x, y, (time * 10.0).floor() as u32);
                d = (d - 0.2).max(0.0) / 0.8;
                d = (d * 2.0 - 1.0).powi(3);
                (
                    (
                        u + d * 0.3,
                        v,
                        time
                    ),
                    (
                        u + d * 0.1,
                        v,
                        time
                    ),
                    (
                        u + d * -0.1,
                        v,
                        time
                    ),
                )
            })
        });

    println!("done!");
}
