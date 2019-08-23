use crate::render::{Rgba, Render, RenderOpt};
use crate::path::Path;

#[derive(Debug)]
pub enum CompositeMode {
    None,
    Normal(Path)
}

pub struct Composite {
    pub layers: Vec<(Box<Render<Rgba>>, CompositeMode)>
}

impl Render<Rgba> for Composite {
    fn sample(&self, u: f64, v: f64, time: f64) -> Rgba {
        let mut value = Rgba::default();
        for (render, cm) in &self.layers {
            value = match cm {
                CompositeMode::None => render.sample(u, v, time),
                CompositeMode::Normal(alpha_path) =>
                    value.normal_blend(&render.sample(u, v, time), alpha_path.get_value(time))
            };
        }
        value
    }
    /*
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
    }*/
}
