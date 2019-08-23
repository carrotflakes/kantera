use crate::render::{Rgba, Render, RenderOpt};

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
