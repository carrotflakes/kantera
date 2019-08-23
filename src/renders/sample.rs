use crate::render::{Rgba, Render, RenderOpt};

pub type Sample<T: Copy> = Box<Fn(f64, f64, f64) -> T>;

impl <T: Copy> Render<T> for Sample<T> {
    fn sample(&self, u: f64, v: f64, time: f64) -> T {
        self(u, v, time)
    }
}
