use crate::render::{Res, Render, RenderOpt};

pub struct Sequence<T: Default, R: Render<T>> {
    pub pages: Vec<(f64, bool, R)>,
    t: std::marker::PhantomData<T>
}

const LARGE_F64: f64 = 100000.0;

impl<T: Default, R: Render<T>> Render<T> for Sequence<T, R> {
    fn sample(&self, u: f64, v: f64, time: f64, res: Res) -> T {
        let mut offset_time = 0.0;
        for i in 0..self.pages.len() {
            let (start, restart, ref render) = self.pages[i];
            if restart {
                offset_time = start;
            }
            let end = self.pages.get(i + 1).map_or(LARGE_F64, |t| t.0);
            if (start..end).contains(&time) {
                return render.sample(u, v, time - offset_time, res);
            }
        }
        T::default()
    }

    fn render(&self, ro: &RenderOpt, buffer: &mut [T]) {
        let RenderOpt {x_range, y_range, frame_range, framerate, ..} = ro;
        let frame_size = ((x_range.end - x_range.start) * (y_range.end - y_range.start)) as usize;
        let mut offset_frame = 0;

        for i in 0..self.pages.len() {
            let (start, restart, ref render) = self.pages[i];
            let end = self.pages.get(i + 1).map_or(LARGE_F64, |t| t.0);

            if restart {
                offset_frame = (start * *framerate as f64) as i32;
            }

            let left: i32 = (frame_range.start).max((start * *framerate as f64).floor() as i32);
            let right: i32 = frame_range.end.min((end * *framerate as f64).floor() as i32);

            if left >= right {
                continue;
            }
            render.render(&RenderOpt {
                frame_range: left - offset_frame..right - offset_frame,
                ..ro.clone()
            }, &mut buffer[(left - frame_range.start as i32) as usize * frame_size..
                           (right - frame_range.start as i32) as usize * frame_size]);
        }
    }
}

impl<T: Default, R: Render<T>> Sequence<T, R> {
    pub fn new() -> Self {
        Sequence {
            pages: vec![],
            t: std::marker::PhantomData
        }
    }

    pub fn append(mut self, time: f64, restart: bool, render: R) -> Self {
        self.pages.push((time, restart, render));
        self
    }

}
