extern crate cairo;

mod buffer;
mod image;
mod path;
mod render;
mod export;
mod renders;
mod util;

fn make_image() -> image::Image<render::Rgba> {
    use cairo::{ImageSurface, Format, Context};

    let (width, height) = (640usize, 480usize);
    let mut surface = ImageSurface::create(Format::ARgb32, width as i32, height as i32).unwrap();
    {
        let ctx = Context::new(&surface);
        //ctx.set_source_rgb(1.0, 1.0, 1.0);
        //ctx.paint();

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
    }

    let data = surface.get_data().unwrap();
    let mut vec = Vec::with_capacity(width * height);
    for i in 0..width * height {
        vec.push(render::Rgba(
            data[i * 4 + 2] as f64 / 255.0,
            data[i * 4 + 1] as f64 / 255.0,
            data[i * 4 + 0] as f64 / 255.0,
            data[i * 4 + 3] as f64 / 255.0
        ));
    }
    image::Image {
        width: width,
        height: height,
        vec: vec
    }
}

fn main() {
    use export::{render_to_mp4, render_to_buffer};
    use renders::{
        plain::Plain,
        sequence::Sequence,
        playback::Playback,
        image_render::ImageRender,
        composite::{Composite, CompositeMode},
        transform::Transform,
        sample::Sample
    };
    use path::{Path, PointType};
    use util::hsl_to_rgb;

    let image = make_image();
    let buffer = render_to_buffer(
        &render::RenderOpt {
            u_range: 0.0..1.0,
            u_res: 20,
            v_range: 0.0..1.0,
            v_res: 20,
            frame_range: 0..100,
            framerate: 4,
        },
        &render::Dummy());

    render_to_mp4(
        10.0,
        640,
        480,
        30,
        &Sequence::new()
            .append(
                0.0,
                true,
                Box::new(Transform {
                    render: Box::new(Playback {buffer: Box::new(buffer)}),
                    transformer: Box::new(|u, v, time| {
                        //let rad = time * 2.0;
                        (
                            //(u - 0.5) * rad.cos() + (v - 0.5) * rad.sin() + 0.5,
                            //(v - 0.5) * rad.cos() - (u - 0.5) * rad.sin() + 0.5,
                            u + (time * 15.0 + v * 40.0).sin() * 0.02 * (time - 1.0).max(0.0),
                            v,
                            time
                        )
                    })
                })
            )
            .append(
                3.0,
                true,
                Box::new(Composite {
                    layers: vec![
                        (
                            //Box::new(Plain(render::Rgba(0.0, 0.0, 1.0, 1.0))),
                            Box::new(Box::new(|u: f64, v: f64, time: f64| {
                                let (r, g, b) = hsl_to_rgb(
                                    v * 0.2 + 0.5,
                                    1.0,
                                    ((u * 10.0).sin() + (v * 10.0).sin() + time).cos() * 0.25 + 0.5);
                                render::Rgba(r, g, b, 1.0)
                            }) as Sample<render::Rgba>),
                            CompositeMode::None
                        ),
                        (
                            Box::new(ImageRender {image: Box::new(image)}),
                            CompositeMode::Normal(
                                Path::new(0.0)
                                    .append(1.0, 1.0, PointType::Linear)
                            )
                        )
                    ]
                })
            ));

    println!("done!");
}
