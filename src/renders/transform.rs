use crate::render::{Render, RenderOpt};

pub struct Transform<T> {
    pub render: Box<Render<T>>,
    pub transformer: Box<Fn(f64, f64, f64) -> (f64, f64, f64)>
}

impl <T> Render<T> for Transform<T> {
    fn sample(&self, u: f64, v: f64, time: f64) -> T {
        let (u, v, time) = (self.transformer)(u, v, time);
        self.render.sample(u, v, time)
    }
}

pub fn camera_shake(size: f64) -> Box<Fn(f64, f64, f64) -> (f64, f64, f64)> {
    Box::new(move |u, v, time| {
        let r = time;
        (
            u + (r * 0.523 + (r * 2.0).sin() * 3.0).sin() * r.cos() * size,
            v + (r * 0.525 + (r * 2.1).sin() * 3.0).sin() * (r * 1.001).cos() * size,
            time
        )
    })
}
