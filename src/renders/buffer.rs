use crate::render::{Rgba, Render, RenderOpt, Buffer};

pub enum {
    interpolation: 
}

pub struct Replay<T> {
    pub buffer: Box<Buffer>,
    pub interpolation: 
}

impl <T> Render<T> for Buffer<T> {
    fn sample(&self, u: f64, v: f64, time: f64) -> T {
        if (0.0..=1.0).contains(u) && (0.0..=1.0).contains(v) {
            self.child.sample(u, v, time)
        } else {
            match self.frame_type {
                Constant(t) => t,
                Repeat => self.child.sample(u, v, time),
                Reflect => self.child.sample(u, v, time)
            }
        }
    }

    fn render(&self, ro: RenderOpt, buffer: &mut [T]) {
        //let RenderOpt {u_res, v_res, frame_range, framerate, ..} = ro;
        self.child.render(ro, buffer);
    }
}
