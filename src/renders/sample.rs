use crate::render::{Res, Render};

pub struct Sample<T>(Box<dyn Fn(f64, f64, f64, Res) -> T>);

impl<T> Sample<T> {
    pub fn new(f: Box<dyn Fn(f64, f64, f64, Res) -> T>) -> Sample<T> {
        Sample(f)
    }
}

impl <T: Copy> Render<T> for Sample<T> {
    fn sample(&self, u: f64, v: f64, time: f64, res: Res) -> T {
        self.0(u, v, time, res)
    }
}
