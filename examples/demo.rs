extern crate kantera;

use kantera::pixel::Rgba;
use kantera::render::{Render, RenderOpt, Dummy};

fn make_image() -> kantera::image::Image<Rgba> {
    let (width, height) = (640, 480);
    kantera::cairo::render_image(width, height, &|ctx| {
        ctx.move_to(0.0, 0.0);
        ctx.line_to(width as f64, height as f64);
        ctx.set_source_rgb(1.0, 0.0, 0.0);
        ctx.stroke().unwrap();

        ctx.set_font_size(60.0);
        let text = "kantera";
        let ext = ctx.text_extents(text).unwrap();
        ctx.move_to((width as f64 - ext.width()) / 2.0, (height as f64 + ext.height()) / 2.0);
        ctx.set_source_rgb(0.9, 0.9, 0.9);
        ctx.show_text(text).unwrap();
    })
}

fn main() {
    use std::rc::Rc;
    use kantera::export::{render_to_mp4, render_to_buffer};
    use kantera::renders::{
        sequence::Sequence,
        playback::Playback,
        image_render::{ImageRender, Sizing},
        composite::{Composite, CompositeMode},
        transform::{Transform, camera_shake},
        sample::Sample,
        bokeh::Bokeh,
        frame::{Frame, FrameType},
        time_extrapolate::{TimeExtrapolate, ExtrapolationType},
        rgb_transform::{RgbTransform}
    };
    use kantera::path::{Path, Point};
    use kantera::util::hsl_to_rgb;

    let image = Rc::new(make_image());
    let buffer = render_to_buffer(
        &RenderOpt {
            x_range: 0..20,
            y_range: 0..20,
            res_x: 20,
            res_y: 20,
            frame_range: 0..100,
            framerate: 4,
        },
        &Dummy());

    let buffer2 = render_to_buffer(
        &RenderOpt {
            x_range: 0..640,
            y_range: 0..480,
            res_x: 640,
            res_y: 480,
            frame_range: 0..30 * 7,
            framerate: 30
        },
        &Composite::<Box<dyn Render<Rgba>>> {
            layers: vec![
                (
                    //Box::new(Plain(Rgba(0.0, 0.0, 1.0, 1.0))),
                    Box::new(Sample::new(Box::new(|u: f64, v: f64, time: f64, _: (usize, usize)| {
                        let (r, g, b) = hsl_to_rgb(
                            v * 0.2 + 0.5,
                            1.0,
                            ((u * 10.0).sin() + (v * 10.0).sin() + time).cos() * 0.25 + 0.5);
                        Rgba(r, g, b, 1.0)
                    }))),
                    CompositeMode::None
                ),
                (
                    Box::new(Bokeh::<Box<dyn Render<Rgba>>> {
                        render: Box::new(ImageRender {
                            image: image.clone(),
                            sizing: Sizing::Fit,
                            default: Rgba(0.0, 0.0, 0.0, 0.0),
                            interpolation: kantera::interpolation::Bilinear
                        }),
                        max_size: 10,
                        size: Rc::new(Path::new(0.0)
                            .append(6.0, 0.0, Point::Constant)
                            .append(1.0, 10.0, Point::Linear)
                    )}),
                    CompositeMode::Normal(
                        Rc::new(Path::new(0.0).append(1.0, 1.0, Point::Linear))
                    )
                )
            ]
        }
    );

    render_to_mp4(
        10.5,
        640,
        480,
        30,
        1,
        "demo.mp4",
        &Sequence::<Rgba, Box<dyn Render<Rgba>>>::new()
            .append(
                0.0,
                true,
                Box::new(Transform::<Rgba, Box<dyn Render<Rgba>>>::new(
                    Box::new(Frame::<Rgba, Box<dyn Render<Rgba>>> {
                        render: Box::new(Playback {buffer: Box::new(buffer)}),
                        frame_type: FrameType::Repeat
                    }),
                    Box::new(|u, v, time, _| {
                        //let rad = time * 2.0;
                        (
                            //(u - 0.5) * rad.cos() + (v - 0.5) * rad.sin() + 0.5,
                            //(v - 0.5) * rad.cos() - (u - 0.5) * rad.sin() + 0.5,
                            u + (time * 15.0 + v * 40.0).sin() * 0.1 * (time - 1.0).max(0.0),
                            v,
                            time
                        )
                    })
                ))
            )
            .append(
                3.0,
                true,
                Box::new(RgbTransform::<Box<dyn Render<Rgba>>> {
                    render: Box::new(TimeExtrapolate::<Rgba, Box<dyn Render<Rgba>>> {
                        duration: buffer2.frame_num as f64 / buffer2.framerate as f64,
                        render: Box::new(Playback::from(buffer2)),
                        extrapolation_type: ExtrapolationType::Extend
                    }),
                    transformer: Box::new({
                        let cs = camera_shake(0.05);
                        move |u, v, t, r| {
                            (cs(u, v, t, r), cs(u, v, t - 0.05, r), cs(u, v, t - 0.1, r))
                        }
                    })
                })
            ));

    println!("done!");
}
