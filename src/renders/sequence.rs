use crate::render::{Rgba, Render, RenderOpt};

pub struct Sequence<T> {
    pub first: Box<Render<T>>,
    pub second: Box<Render<T>>,
    pub time: f64
}

impl <T> Render<T> for Sequence<T> {
    fn sample(&self, u: f64, v: f64, time: f64) -> T {
        if time < self.time {
            &self.first
        } else {
            &self.second
        }.sample(u, v, time)
    }

    fn render(&self, ro: RenderOpt, buffer: &mut [T]) {
        let RenderOpt {u_res, v_res, frame_range, framerate, ..} = ro;
        let frame_size = u_res * v_res;

        {
            let sep = frame_range.end.min((self.time * framerate as f64).floor() as i32);
            if frame_range.start < sep {
                self.first.render(RenderOpt {
                    u_range: ro.u_range.start..ro.u_range.end,
                    u_res: ro.u_res,
                    v_range: ro.v_range.start..ro.v_range.end,
                    v_res: ro.v_res,
                    frame_range: frame_range.start..sep,
                    framerate: framerate
                }, &mut buffer[..(sep - frame_range.start as i32) as usize * frame_size]);
            }
        }
        {
            let sep = frame_range.start.max((self.time * framerate as f64).floor() as i32);
            if sep <= frame_range.end {
                self.second.render(RenderOpt {
                    u_range: ro.u_range.start..ro.u_range.end,
                    u_res: ro.u_res,
                    v_range: ro.v_range.start..ro.v_range.end,
                    v_res: ro.v_res,
                    frame_range: sep..frame_range.end,
                    framerate: framerate
                }, &mut buffer[(sep - frame_range.start as i32) as usize * frame_size..]);
            }
        }
    }
}
