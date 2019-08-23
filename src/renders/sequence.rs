use crate::render::{Rgba, Render, RenderOpt};

pub struct Sequence<T: Default> {
    pub pages: Vec<(f64, Box<Render<T>>)>
}

impl <T: Default> Render<T> for Sequence<T> {
    fn sample(&self, u: f64, v: f64, time: f64) -> T {
        match self.get_render(time) {
            Some(render) => render.sample(u, v, time),
            None => T::default()
        }
    }

    fn render(&self, ro: &RenderOpt, buffer: &mut [T]) {
        let RenderOpt {u_res, v_res, frame_range, framerate, ..} = ro;
        let frame_size = u_res * v_res;

        for i in 0..self.pages.len() {
            let (start, ref render) = self.pages[i];
            let end = self.pages.get(i + 1).map_or(100000.0, |(end, _)| *end);

            let left: i32 = (frame_range.start).max((start * *framerate as f64).floor() as i32);
            let right: i32 = frame_range.end.min((end * *framerate as f64).floor() as i32);

            if left >= right {
                continue;
            }
            render.render(&RenderOpt {
                u_range: ro.u_range.start..ro.u_range.end,
                u_res: ro.u_res,
                v_range: ro.v_range.start..ro.v_range.end,
                v_res: ro.v_res,
                frame_range: left..right, // TODO: restart option
                framerate: *framerate
            }, &mut buffer[(left - frame_range.start as i32) as usize * frame_size..
                           (right - frame_range.start as i32) as usize * frame_size]);
        }
    }
}

impl <T: Default> Sequence<T> {
    #[inline(always)]
    fn get_render(&self, time: f64) -> Option<&Box<Render<T>>> {
        for (page_time, render) in self.pages.iter().rev() {
            if *page_time <= time {
                return Some(render);
            }
        }
        None
    }
}
