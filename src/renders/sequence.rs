use crate::render::{Res, Render, RenderOpt};

pub struct Sequence<T: Default> {
    pub pages: Vec<(f64, bool, Box<dyn Render<T>>)>
}

const LARGE_F64: f64 = 100000.0;

impl <T: Default> Render<T> for Sequence<T> {
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
        let RenderOpt {u_res, v_res, frame_range, framerate, ..} = ro;
        let frame_size = u_res * v_res;
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
                .. *ro
            }, &mut buffer[(left - frame_range.start as i32) as usize * frame_size..
                           (right - frame_range.start as i32) as usize * frame_size]);
        }
    }
}

impl <T: Default> Sequence<T> {
    pub fn new() -> Self {
        Sequence {
            pages: vec![]
        }
    }

    pub fn append(mut self, time: f64, restart: bool, render: Box<dyn Render<T>>) -> Self {
        self.pages.push((time, restart, render));
        self
    }

}
