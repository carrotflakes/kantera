extern crate kantera;

use kantera::pixel::Rgba;
use kantera::export::render_to_mp4;
use kantera::renders::sample::Sample;
use kantera::util::noise;

fn main() {
    render_to_mp4(
        5.0, 320, 240, 30, 1,
        "noise.mp4",
        &(Box::new(|u: f64, v: f64, time: f64, (w, h): (usize, usize)| {
            let v = noise(u / 10.0 * w as f64, v / 10.0 * h as f64, time) * 0.5 + 0.5;
            Rgba(v, v, v, 1.0)
        }) as Sample<Rgba>));

    println!("done!");
}
