use crate::render::{Res, Render, RenderOpt};

pub struct FunctionalRender<T: Copy>(pub Box<dyn Fn(&RenderOpt, f64, &mut [T])>);

impl <T: Copy> Render<T> for FunctionalRender<T> {
    fn sample(&self, _u: f64, _v: f64, _time: f64, _res: Res) -> T {
        panic!("FunctionalRender cannot sample");
    }

    fn render(&self, ro: &RenderOpt, buffer: &mut [T]) {
        let RenderOpt {u_res, v_res, frame_range, framerate, ..} = ro;
        let frame_size = *u_res * *v_res;
        for f in 0..(frame_range.end - frame_range.start) as usize {
            let time = (frame_range.start as f64 + f as f64) / *framerate as f64;
            (self.0)(ro, time, &mut buffer[f * frame_size..(f + 1) * frame_size]);
        }
    }
}
