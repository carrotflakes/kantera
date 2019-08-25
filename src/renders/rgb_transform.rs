use crate::render::{Render};
use crate::pixel::Rgba;

type Uvt = (f64, f64, f64);

pub struct RgbTransform {
    pub render: Box<Render<Rgba>>,
    pub transformer: Box<Fn(f64, f64, f64) -> (Uvt, Uvt, Uvt)>
}

impl Render<Rgba> for RgbTransform {
    fn sample(&self, u: f64, v: f64, time: f64) -> Rgba {
        let (r, g, b) = (self.transformer)(u, v, time);
        let r = self.render.sample(r.0, r.1, r.2).0;
        let g = self.render.sample(g.0, g.1, g.2).1;
        let b = self.render.sample(b.0, b.1, b.2).2;
        Rgba(r, g, b, 1.0)
    }
}

