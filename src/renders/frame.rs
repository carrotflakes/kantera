use crate::render::{Rgba, Render, RenderOpt};

pub enum FrameType<T> {
    Constant(T),
    Repeat,
    Reflect
}

pub struct Frame<T> {
    pub child: Box<Render<T>>,
    pub frame_type: FrameType<T>
}

impl <T> Render<T> for Frame<T> {
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

    fn render(&self, ro: &RenderOpt, buffer: &mut [T]) {
        //let RenderOpt {u_res, v_res, frame_range, framerate, ..} = ro;
        self.child.render(ro, buffer);
    }
}
