use crate::render::{Res, Render, RenderOpt};
use std::marker::PhantomData;

pub struct Clip<T, R: Render<T>> {
    pub render: R,
    pub start: f64,
    pub end: f64,
    pub t: PhantomData<T>
}

impl <T, R: Render<T>> Render<T> for Clip<T, R> {
    fn sample(&self, u: f64, v: f64, time: f64, res: Res) -> T {
        let time = time + self.start;
        self.render.sample(u, v, time, res)
    }

    fn render(&self, ro: &RenderOpt, buffer: &mut [T]) {
        let dframe = (ro.framerate as f64 * self.start) as i32;
        let ro = RenderOpt {
            frame_range: ro.frame_range.start + dframe..ro.frame_range.end + dframe,
            ..ro.clone()
        };
        self.render.render(&ro, buffer);
    }

    fn duration(&self) -> f64 {
        self.end - self.start
    }
}

impl<T, R: Render<T>> Clip<T, R> {
    pub fn new(render: R, start: f64, end: f64) -> Self {
        assert!(start <= end);
        Clip {
            render,
            start,
            end,
            t: PhantomData
        }
    }
}
