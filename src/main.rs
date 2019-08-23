extern crate cairo;

mod buffer;
mod image;
mod path;
mod render;
mod export;
mod renders;

use cairo::{ImageSurface, Format, Context};
use export::{render_to_mp4, render_to_buffer};

fn make_image() -> image::Image<render::Rgba> {
    let (width, height) = (320usize, 240usize);
    let mut surface = ImageSurface::create(Format::ARgb32, width as i32, height as i32).unwrap();
    {
        let ctx = Context::new(&surface);
        //ctx.set_source_rgb(1.0, 1.0, 1.0);
        //ctx.paint();

        ctx.move_to(0.0, 0.0);
        ctx.line_to(width as f64, height as f64);
        ctx.set_source_rgb(1.0, 0.0, 0.0);
        ctx.stroke();

        ctx.set_font_size(30.0);
        let text = "kantera";
        let ext = ctx.text_extents(text);
        ctx.move_to((width as f64 - ext.width) / 2.0, (height as f64 + ext.height) / 2.0);
        ctx.set_source_rgb(0.3, 0.3, 0.3);
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
    let image = make_image();
    let buffer = render_to_buffer(
        render::RenderOpt {
            u_range: 0.0..1.0,
            u_res: 20,
            v_range: 0.0..1.0,
            v_res: 20,
            frame_range: 0..100,
            framerate: 4,
        },
        &render::Dummy());

    use renders::{
        playback::Playback,
        composite::{Composite, CompositeMode}
    };

    render_to_mp4(
        5.0,
        &renders::sequence::Sequence {
            first: Box::new(renders::playback::Playback {buffer: Box::new(buffer)}),
            //second: Box::new(renders::plain::Plain(render::Rgba(1.0, 0.0, 0.0, 1.0))),
            //second: Box::new(renders::image_render::ImageRender {image: Box::new(image)}),
            second: Box::new(renders::composite::Composite {
                layers: vec![
                    (
                        Box::new(renders::plain::Plain(render::Rgba(0.0, 0.0, 1.0, 1.0))),
                        renders::composite::CompositeMode::None
                    ),
                    (
                        Box::new(renders::image_render::ImageRender {image: Box::new(image)}),
                        renders::composite::CompositeMode::Normal(0.9)
                    )
                ]
            }),
            time: 3.0
        });
}
