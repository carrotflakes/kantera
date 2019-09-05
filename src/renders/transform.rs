use crate::render::{Res, Render};
use crate::util::noise;

pub struct Transform<T> {
    pub render: Box<Render<T>>,
    pub transformer: Box<Fn(f64, f64, f64, Res) -> (f64, f64, f64)>
}

impl <T> Render<T> for Transform<T> {
    fn sample(&self, u: f64, v: f64, time: f64, res: Res) -> T {
        let (u, v, time) = (self.transformer)(u, v, time, res);
        self.render.sample(u, v, time, res)
    }
}

pub fn camera_shake(size: f64) -> Box<Fn(f64, f64, f64, Res) -> (f64, f64, f64)> {
    Box::new(move |u, v, time, _| {
        let r = time;
        (
            u + (r * 0.523 + (r * 2.0).sin() * 3.0).sin() * r.cos() * size,
            v + (r * 0.525 + (r * 2.1).sin() * 3.0).sin() * (r * 1.001).cos() * size,
            time
        )
    })
}

pub fn camera_shake2(size: f64, time_scale: f64) -> Box<Fn(f64, f64, f64, Res) -> (f64, f64, f64)> {
    Box::new(move |u, v, time, res| {
        let r = time;
        (
            u + noise(0.0, 0.0, time * time_scale) / res.0 as f64 * size,
            v + noise(0.5, 2.0, time * time_scale) / res.1 as f64 * size,
            time
        )
    })
}
