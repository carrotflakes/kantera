extern crate kantera;

use kantera::pixel::Rgba;
use kantera::render::{Range, Render, RenderOpt, Dummy};

fn make_image() -> kantera::image::Image<Rgba> {
    let (width, height) = (640, 480);
    kantera::cairo::render_image(width, height, &|ctx| {
        ctx.move_to(0.0, 0.0);
        ctx.line_to(width as f64, height as f64);
        ctx.set_source_rgb(1.0, 0.0, 0.0);
        ctx.stroke();

        ctx.set_font_size(60.0);
        let text = "kantera";
        let ext = ctx.text_extents(text);
        ctx.move_to((width as f64 - ext.width) / 2.0, (height as f64 + ext.height) / 2.0);
        ctx.set_source_rgb(0.9, 0.9, 0.9);
        ctx.show_text(text);
    })
}

fn main() {
    use std::rc::Rc;
    use kantera::export::{render_to_mp4, render_to_buffer};
    use kantera::renders::{
        plain::Plain,
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
    use kantera::path::{Path, PointType};
    use kantera::util::hsl_to_rgb;

    let image = Rc::new(make_image());
    let buffer = render_to_buffer(
        &RenderOpt {
            u_range: Range::unit(),
            u_res: 20,
            v_range: Range::unit(),
            v_res: 20,
            frame_range: 0..100,
            framerate: 4,
        },
        &Dummy());

    let buffer2 = render_to_buffer(
        &RenderOpt {
            u_range: Range::unit(),
            u_res: 640,
            v_range: Range::unit(),
            v_res: 480,
            frame_range: 0..30 * 7,
            framerate: 30
        },
        &Composite {
            layers: vec![
                (
                    //Box::new(Plain(Rgba(0.0, 0.0, 1.0, 1.0))),
                    Box::new(Box::new(|u: f64, v: f64, time: f64, _: (usize, usize)| {
                        let (r, g, b) = hsl_to_rgb(
                            v * 0.2 + 0.5,
                            1.0,
                            ((u * 10.0).sin() + (v * 10.0).sin() + time).cos() * 0.25 + 0.5);
                        Rgba(r, g, b, 1.0)
                    }) as Sample<Rgba>),
                    CompositeMode::None
                ),
                (
                    Box::new(Bokeh {
                        render: Box::new(ImageRender {image: image.clone(), sizing: Sizing::Fit}),
                        max_size: 10,
                        size_path: Path::new(0.0)
                            .append(6.0, 0.0, PointType::Constant)
                            .append(1.0, 10.0, PointType::Linear)
                    }),
                    CompositeMode::Normal(
                        Path::new(0.0)
                            .append(1.0, 1.0, PointType::Linear)
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
        &Sequence::new()
            .append(
                0.0,
                true,
                Box::new(Transform {
                    render: Box::new(Frame {
                        render: Box::new(Playback {buffer: Box::new(buffer)}),
                        frame_type: FrameType::Repeat
                    }),
                    transformer: Box::new(|u, v, time, _| {
                        //let rad = time * 2.0;
                        (
                            //(u - 0.5) * rad.cos() + (v - 0.5) * rad.sin() + 0.5,
                            //(v - 0.5) * rad.cos() - (u - 0.5) * rad.sin() + 0.5,
                            u + (time * 15.0 + v * 40.0).sin() * 0.1 * (time - 1.0).max(0.0),
                            v,
                            time
                        )
                    })
                })
            )
            .append(
                3.0,
                true,
                Box::new(RgbTransform {
                    render: Box::new(TimeExtrapolate {
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
