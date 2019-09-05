use crate::render::{Res, Render, RenderOpt};

pub struct Plain<T: Copy>(pub T);

impl <T: Copy> Render<T> for Plain<T> {
    fn sample(&self, _u: f64, _v: f64, _time: f64, res: Res) -> T {
        self.0
    }

    fn render(&self, ro: &RenderOpt, buffer: &mut [T]) {
        let RenderOpt {u_res, v_res, frame_range, framerate, ..} = ro;
        for f in frame_range.start..frame_range.end {
            for v in 0..*v_res {
                for u in 0..*u_res {
                    buffer[(f - frame_range.start) as usize * u_res * v_res + v * u_res + u] =
                        self.0;
                }
            }
        }
    }
}
