use crate::render::{Res, Render};

pub type Sample<T> = Box<dyn Fn(f64, f64, f64, Res) -> T>;

impl <T: Copy> Render<T> for Sample<T> {
    fn sample(&self, u: f64, v: f64, time: f64, res: Res) -> T {
        self(u, v, time, res)
    }
}
