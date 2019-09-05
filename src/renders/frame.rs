use crate::render::{Res, Render};

#[derive(Debug, Copy, Clone)]
pub enum FrameType<T: Copy> {
    Constant(T),
    Extend,
    Repeat,
    Reflect
}

pub struct Frame<T: Copy> {
    pub render: Box<Render<T>>,
    pub frame_type: FrameType<T>
}

impl <T: Copy> Render<T> for Frame<T> {
    fn sample(&self, u: f64, v: f64, time: f64, res: Res) -> T {
        if (0.0..=1.0).contains(&u) && (0.0..=1.0).contains(&v) {
            self.render.sample(u, v, time, res)
        } else {
            match &self.frame_type {
                FrameType::Constant(t) => *t,
                FrameType::Extend =>
                    self.render.sample(
                        u.max(0.0).min(0.9999999),
                        v.max(0.0).min(0.9999999),
                        time, res),
                FrameType::Repeat => self.render.sample(
                    u - u.floor(),
                    v - v.floor(),
                    time, res),
                FrameType::Reflect => self.render.sample(
                    if (u.floor() as i32) % 2 == 0 {u - u.floor()} else {1.0 - u + u.floor()},
                    if (v.floor() as i32) % 2 == 0 {v - v.floor()} else {1.0 - v + v.floor()},
                    time, res)
            }
        }
    }

    // TODO render?
}
