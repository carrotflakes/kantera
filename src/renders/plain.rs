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
        let RenderOpt {u_res, v_res, frame_range, framerate, ..} = ro;
        for f in frame_range.start..frame_range.end {
            for v in 0..*v_res {
                for u in 0..*u_res {
                    buffer[(f - frame_range.start) as usize * u_res * v_res + v * u_res + u] =
                        self.0.get_value(f as f64 / *framerate as f64);
                }
            }
        }
    }
}
