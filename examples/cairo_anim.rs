extern crate kantera;

use std::f64::consts::PI;
use kantera::export::render_to_mp4;
use kantera::renders::playback::Playback;
use kantera::cairo::render_buffer;


fn main() {

    let buffer = render_buffer(320, 240, 30, &|mut ctx| {
        //let font_face = FontFace::toy_create(
        //    "monospace", FontSlant::Normal, FontWeight::Normal);
       // ctx.set_font_face(&font_face);

        for i in ["5", "4", "3", "2", "1"].iter() {
            ctx.set_source_rgb(0.1, 0.1, 0.1);
            ctx.paint().unwrap();

            for j in 0..30 {
                ctx.move_to(160.0, 120.0);
                ctx.line_to(((j as f64 / 30.0 - 0.25) * PI * 2.0).cos() * 100.0 + 160.0,
                            ((j as f64 / 30.0 - 0.25) * PI * 2.0).sin() * 100.0 + 120.0);
                ctx.set_source_rgb(1.0, 0.0, 0.0);
                ctx.stroke().unwrap();

                ctx.set_font_size(60.0);
                let text = i;
                let ext = ctx.text_extents(text).unwrap();
                ctx.move_to((320 as f64 - ext.width()) / 2.0, (240 as f64 + ext.height()) / 2.0);
                ctx.set_source_rgb(0.9, 0.9, 0.9);
                ctx.show_text(text).unwrap();

                ctx.push();
            }
        }
    });

    render_to_mp4(
        5.0,
        320,
        240,
        30,
        1,
        "cairo_anim.mp4",
        &Playback {buffer: Box::new(buffer)});

    println!("done!");
}
