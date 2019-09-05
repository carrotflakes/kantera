use crate::render::{Res, Render};

#[derive(Debug, Copy, Clone)]
pub enum ExtrapolationType<T: Copy> {
    None,
    Constant(T),
    Extend,
    Repeat,
    Reflect
}

pub struct TimeExtrapolate<T: Copy> {
    pub render: Box<Render<T>>,
    pub duration: f64,
    pub extrapolation_type: ExtrapolationType<T>
}

impl <T: Copy> Render<T> for TimeExtrapolate<T> {
    fn sample(&self, u: f64, v: f64, time: f64, res: Res) -> T {
        match &self.extrapolation_type {
            ExtrapolationType::None => self.render.sample(u, v, time, res),
            ExtrapolationType::Constant(t) =>
                if (0.0..self.duration).contains(&time) {
                    self.render.sample(u, v, time, res)
                } else {
                    *t
                },
            ExtrapolationType::Extend =>
                self.render.sample(
                    u, v, time.max(0.0).min(self.duration - 0.00001), res), // FIXME
            ExtrapolationType::Repeat =>
                self.render.sample(
                    u, v, time % self.duration, res),
            ExtrapolationType::Reflect =>
                self.render.sample(
                    u, v,
                    if (time / self.duration).floor() as i32 % 2 == 0 {
                        time - (time / self.duration).floor() * self.duration
                    } else {
                        self.duration - time + (time / self.duration).floor() * self.duration
                    },
                    res)
        }
    }

    // TODO render
}
