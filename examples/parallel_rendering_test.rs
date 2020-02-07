extern crate kantera;

use kantera::pixel::Rgba;
use kantera::export::{render_to_buffer, render_to_buffer_parallel};
use kantera::buffer::Buffer;
use kantera::renders::sample::Sample;
use kantera::render::{Range, Render, RenderOpt};
use kantera::util::noise;

fn main() {
    let ro = RenderOpt {
        u_range: Range::unit(),
        u_res: 3840 / 4,
        v_range: Range::unit(),
        v_res: 2160 / 4,
        frame_range: 0..30 * 1,
        framerate: 30,
    };
    let render = Sample::new(Box::new(|u: f64, v: f64, time: f64, (w, h): (usize, usize)| {
        Rgba(
            noise(u / 10.0 * w as f64, v / 10.0 * h as f64, time) * 0.5 + 0.5,
            noise(u / 10.0 * w as f64, v / 10.0 * h as f64, time) * 0.5 + 0.5,
            noise(u / 10.0 * w as f64, v / 10.0 * h as f64, time) * 0.5 + 0.5, 1.0)
    }));
    for _ in 0..5 {
        let start = std::time::Instant::now();
        render_to_buffer(&ro, &render);
        println!("{:?}", start.elapsed());
        let start = std::time::Instant::now();
        render_to_buffer_parallel(&ro, unsafe { std::mem::transmute::<&dyn Render<Rgba>, &'static (dyn Render<Rgba> + Send + Sync)>(&render) }) as Buffer<Rgba>;
        println!("{:?}", start.elapsed());
    }

    println!("done!");
}
