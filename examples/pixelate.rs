extern crate kantera;

use kantera::pixel::Rgba;
use kantera::export::render_to_mp4;
use kantera::render::Render;
use kantera::renders::{
    sample::Sample,
    map::Map
};
use kantera::util::noise;

fn main() {
    render_to_mp4(
        5.0, 320, 240, 30, 1,
        "pixelate.mp4",
        &Map::<Rgba, Box<dyn Render<Rgba>>> {
            render: Box::new(Sample::new(Box::new(|u: f64, v: f64, time: f64, (w, h): (usize, usize)| {
                let d = ((u - 0.5).powi(2) + (v - 0.5).powi(2)).powf(0.5);
                let (u, v) = ((u - 0.5) * (0.5 + d * 2.0) + 0.5, (v - 0.5) * (0.5 + d * 2.0) + 0.5);
                let x = u / 20.0 * w as f64;
                let y = v / 20.0 * h as f64;
                Rgba(
                    noise(x + time * 5.0, y, 0.0) * 0.5 + 0.5,
                    noise(x + time * 3.0, y, 1.0) * 0.5 + 0.5,
                    noise(x + time * 1.0, y, 2.0) * 0.5 + 0.5,
                    1.0)
            }))),
            map: Box::new(|w, h, buffer| {
                let size = 7;
                for y in 0..(h - 1) / size + 1 {
                    for x in 0..(w - 1) / size + 1 {
                        let mut s = [0.0; 4];
                        for dy in 0..size.min(h - y * size) {
                            let y = y * size + dy;
                            for dx in 0..size.min(w - x * size) {
                                let x = x * size + dx;
                                let p = buffer[y * w + x];
                                s[0] += p.0;
                                s[1] += p.1;
                                s[2] += p.2;
                                s[3] += p.3;
                            }
                        }
                        let p = {
                            let d = (size.min(w - x * size) * size.min(h - y * size)) as f64;
                            Rgba(s[0] / d, s[1] / d, s[2] / d, s[3] / d)
                        };
                        for dy in 0..size.min(h - y * size) {
                            let y = y * size + dy;
                            for dx in 0..size.min(w - x * size) {
                                let x = x * size + dx;
                                buffer[y * w + x] = p;
                            }
                        }
                    }
                }
            })
        });

    println!("done!");
}
