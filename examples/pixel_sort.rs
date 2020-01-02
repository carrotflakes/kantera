extern crate kantera;

use kantera::pixel::Rgba;
use kantera::export::render_to_mp4;
use kantera::render::Render;
use kantera::renders::{
    sample::Sample,
    map::Map
};
use kantera::util::noise;

fn brightness(p: &Rgba) -> f64 {
    //(p.0 + p.1 + p.2) / 3.0
    (p.0 * 100.0 + p.1 * 10.0 + p.2) / 111.0
}

fn rgba_cmp(l: &Rgba, r: &Rgba) -> std::cmp::Ordering {
    //let l = l.0 * 100.0 + l.1 * 10.0 + l.2;
    //let r = r.0 * 100.0 + r.1 * 10.0 + r.2;
    let l = brightness(l);
    let r = brightness(r);
    r.partial_cmp(&l).unwrap()
}

fn main() {
    render_to_mp4(
        5.0, 320, 240, 30, 1,
        "map.mp4",
        &Map::<Rgba, Box<dyn Render<Rgba>>> {
            render: Box::new(Box::new(|u: f64, v: f64, time: f64, (w, h): (usize, usize)| {
                let d = ((u - 0.5).powi(2) + (v - 0.5).powi(2)).powf(0.5);
                let (u, v) = ((u - 0.5) * (0.5 + d * 2.0) + 0.5, (v - 0.5) * (0.5 + d * 2.0) + 0.5);
                let x = u / 20.0 * w as f64;
                let y = v / 20.0 * h as f64;
                Rgba(
                    noise(x + time * 5.0, y, 0.0) * 0.5 + 0.5,
                    noise(x + time * 3.0, y, 1.0) * 0.5 + 0.5,
                    noise(x + time * 1.0, y, 2.0) * 0.5 + 0.5,
                    1.0)
            }) as Sample<Rgba>),
            map: Box::new(|w, h, buffer| {
                for y in 0..h {
                    let mut left = 0;
                    while left < w {
                        while left < w && brightness(&buffer[y * w + left]) > 0.6 {
                            left += 1;
                        }
                        if left == w {
                            break;
                        }
                        let mut right = left + 1;
                        while right < w && brightness(&buffer[y * w + right]) <= 0.6 {
                            right += 1;
                        }
                        (&mut buffer[y * w + left..y * w + right]).sort_by(rgba_cmp);
                        left = right;
                    }
                }
            })
        });

    println!("done!");
}
