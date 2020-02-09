use crate::render::{Res, Render, RenderOpt};
use crate::timed::Timed;
use std::marker::PhantomData;

pub struct Plain<T: Copy, U: Timed<T>>(pub U, PhantomData<T>);

impl<T: Copy, U: Timed<T>> Plain<T, U> {
    pub fn new(u: U) -> Self {
        Plain(u, PhantomData)
    }
}

impl <T: Copy, U: Timed<T>> Render<T> for Plain<T, U> {
    fn sample(&self, _u: f64, _v: f64, time: f64, _res: Res) -> T {
        self.0.get_value(time)
    }

    fn render(&self, ro: &RenderOpt, buffer: &mut [T]) {
        let RenderOpt {x_range, y_range, frame_range, framerate, ..} = ro;
        let x_size = (x_range.end - x_range.start) as usize;
        let y_size = (y_range.end - y_range.start) as usize;
        for f in frame_range.start..frame_range.end {
            for y in 0..y_size {
                for x in 0..x_size {
                    buffer[(f - frame_range.start) as usize * x_size * y_size + y * x_size + x] =
                        self.0.get_value(f as f64 / *framerate as f64);
                }
            }
        }
    }
}
