extern crate kantera;

use kantera::pixel::Rgba;
use kantera::export::render_to_mp4;
use kantera::renders::{
    sample::Sample,
    filter::{Filter, make_gaussian_filter}
};
use kantera::image::Image;
use kantera::util::u32_noise;

fn main() {
    let _filter = Image {
        width: 3,
        height: 3,
        // vec: vec![Rgba(0.0, 0.0, 0.0, 0.0), Rgba(0.0, 0.0, 0.0, 0.0), Rgba(0.0, 0.0, 0.0, 0.0),
        //           Rgba(0.0, 0.0, 0.0, 0.0), Rgba(1.0, 1.0, 1.0, 1.0), Rgba(0.0, 0.0, 0.0, 0.0),
        //           Rgba(0.0, 0.0, 0.0, 0.0), Rgba(0.0, 0.0, 0.0, 0.0), Rgba(0.0, 0.0, 0.0, 0.0)]
        // vec: vec![Rgba(0.1, 0.1, 0.1, 0.0), Rgba(0.1, 0.1, 0.1, 1.0), Rgba(0.1, 0.1, 0.1, 0.0),
        //           Rgba(0.1, 0.1, 0.1, 0.0), Rgba(0.2, 0.2, 0.2, 1.0), Rgba(0.1, 0.1, 0.1, 0.0),
        //           Rgba(0.1, 0.1, 0.1, 0.0), Rgba(0.1, 0.1, 0.1, 1.0), Rgba(0.1, 0.1, 0.1, 0.0)]
        vec: vec![Rgba(-1.0, 0.0, -1.0, 0.0), Rgba(-1.0, 0.0, 2.0, 0.0), Rgba(-1.0, 0.0, -1.0, 0.0),
                  Rgba(2.0, 0.0, -1.0, 0.0), Rgba(3.0, 1.0, 3.0, 1.0), Rgba(2.0, 0.0, -1.0, 0.0),
                  Rgba(-1.0, 0.0, -1.0, 0.0), Rgba(-1.0, 0.0, 2.0, 0.0), Rgba(-1.0, 0.0, -1.0, 0.0)]
    };
    render_to_mp4(
        5.0, 320, 240, 30, 1,
        "filter.mp4",
        &Filter {
            render: Box::new(Box::new(|u: f64, v: f64, time: f64, (w, h): (usize, usize)| {
                let d = ((u - 0.5).powi(2) + (v - 0.5).powi(2)).powf(0.5);
                let (u, v) = ((u - 0.5) * (0.5 + d * 2.0) + 0.5, (v - 0.5) * (0.5 + d * 2.0) + 0.5);
                let x = (u / 10.0 * w as f64 + time * 5.0).floor() as u32;
                let y = (v / 10.0 * h as f64).floor() as u32;
                let v = u32_noise(x, y, 0) as f64 / std::u32::MAX as f64;
                Rgba(v, v, v, 1.0)
            }) as Sample<Rgba>),
            filter: make_gaussian_filter(10, 10, 3.0)
        });

    println!("done!");
}
