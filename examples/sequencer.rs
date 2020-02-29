extern crate kantera;

use kantera::pixel::Rgba;
use kantera::render::Render;
use kantera::export::render_to_mp4;
use kantera::renders::{
    sequencer::Sequencer,
    clip::Clip,
    plain::Plain
};

fn main() {
    let (width, height) = (320, 240);

    render_to_mp4(
        10.0, width, height, 30, 1,
        "sequencer.mp4",
        &Sequencer::<Rgba, Box<dyn Render<Rgba>>>::new(Rgba(0.0, 0.0, 0.0, 1.0))
            .append(
                1.0,
                0,
                Box::new(Clip::new(Plain::new(Rgba(1.0, 0.0, 0.0, 1.0)), 0.0, 1.0)))
            .append(
                2.0,
                0,
                Box::new(Clip::new(Plain::new(Rgba(0.0, 0.0, 1.0, 1.0)), 0.0, 1.0)))
            .append(
                4.0,
                1,
                Box::new(Clip::new(Plain::new(Rgba(1.0, 0.0, 0.0, 0.5)), 0.0, 5.0)))
            .append(
                5.0,
                2,
                Box::new(Clip::new(Plain::new(Rgba(0.0, 0.0, 1.0, 0.5)), 0.0, 1.0)))
            .append(
                7.0,
                0,
                Box::new(Clip::new(Plain::new(Rgba(0.0, 0.0, 1.0, 0.5)), 0.0, 1.0)))
    );

    println!("done!");
}
