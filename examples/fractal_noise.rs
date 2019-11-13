extern crate kantera;

use kantera::pixel::Rgba;
use kantera::export::render_to_mp4;
use kantera::renders::sample::Sample;
use kantera::util::u32_noise;

fn fractal_noise(n: usize, x: u32, y: u32, z: u32) -> u32 {
    let mut v = 0;
    for i in 0..n {
        v += u32_noise(x >> i, y >> i, z) / n as u32;
    }
    v
}

fn main() {
    render_to_mp4(
        5.0, 320, 240, 30, 1,
        "fractal_noise.mp4",
        &(Box::new(|u: f64, v: f64, time: f64, (w, h): (usize, usize)| {
            let x = (u / 2.0 * w as f64).floor() as u32;
            let y = (v / 2.0 * h as f64).floor() as u32;
            let v = fractal_noise(10, x, y, (time * 10.0).floor() as u32) as f64 / std::u32::MAX as f64;
            Rgba(v, v, v, 1.0)
        }) as Sample<Rgba>));

    println!("done!");
}
