use crate::render::{Res, Render};
use crate::util::noise;

pub type TransformFn = Fn(f64, f64, f64, Res) -> (f64, f64, f64);

pub struct Transform<T> {
    pub render: Box<Render<T>>,
    pub transformer: Box<TransformFn>
}

impl <T> Render<T> for Transform<T> {
    fn sample(&self, u: f64, v: f64, time: f64, res: Res) -> T {
        let (u, v, time) = (self.transformer)(u, v, time, res);
        self.render.sample(u, v, time, res)
    }
}

pub fn camera_shake(size: f64) -> Box<TransformFn> {
    Box::new(move |u, v, time, _| {
        let r = time;
        (
            u + (r * 0.523 + (r * 2.0).sin() * 3.0).sin() * r.cos() * size,
            v + (r * 0.525 + (r * 2.1).sin() * 3.0).sin() * (r * 1.001).cos() * size,
            time
        )
    })
}

pub fn camera_shake2(size: f64, time_scale: f64) -> Box<TransformFn> {
    Box::new(move |u, v, time, res| {
        let r = time;
        (
            u + noise(0.0, 0.0, time * time_scale) / res.0 as f64 * size,
            v + noise(0.5, 2.0, time * time_scale) / res.1 as f64 * size,
            time
        )
    })
}

#[derive(Debug, Copy, Clone)]
pub struct Mat(f64, f64, f64, f64, f64, f64);

impl Mat {
    pub fn new() -> Mat {
        Mat(1.0, 0.0, 0.0, 0.0, 1.0, 0.0)
    }

    pub fn translate(&self, x: f64, y: f64) -> Mat {
        Mat(self.0, self.1, - x * self.0 - y * self.1 + self.2,
            self.3, self.4, - x * self.3 - y * self.4 + self.5)
    }

    pub fn scale(&self, x: f64, y: f64) -> Mat {
        Mat(self.0 / x, self.1 / x, self.2, self.3 / y, self.4 / y, self.5)
    }

    pub fn rotate(&self, rad: f64) -> Mat {
        let (sin, cos) = rad.sin_cos();
        Mat(self.0 * cos + self.1 * sin, self.0 * -sin + self.1 * cos,
            self.2,
            self.3 * cos + self.4 * sin, self.3 * -sin + self.4 * cos,
            self.5)
    }

    //pub fn skew(&self, x: f64, y: f64) -> Mat {
    //}

    pub fn get_transformer(&self) -> Box<TransformFn> {
        let mat = self.clone();
        Box::new(move |u, v, time, res| {
            let x = u * res.0 as f64;
            let y = v * res.1 as f64;
            (
                (x * mat.0 + y * mat.1 + mat.2) / res.0 as f64,
                (x * mat.3 + y * mat.4 + mat.5) / res.1 as f64,
                time
            )
        })
    }
}
