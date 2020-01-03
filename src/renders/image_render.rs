use std::rc::Rc;
use crate::render::{Res, Render};
use crate::image::Image;
use crate::lerp::Lerp;
use crate::interpolation::Interpolation;

#[derive(Debug, Copy, Clone)]
pub enum Sizing {
    Fit,
    Contain,
    Cover,
    DotByDot,
}

pub struct ImageRender<T: Lerp + Copy, U: Interpolation<T>> {
    pub image: Rc<Image<T>>,
    pub sizing: Sizing,
    pub default: T,
    pub interpolation: U
}

impl <T: Lerp + Copy, U: Interpolation<T>> Render<T> for ImageRender<T, U> {
    fn sample(&self, u: f64, v: f64, _time: f64, res: Res) -> T {
        let width = self.image.width;
        let height = self.image.height;
        let (x, y) = match self.sizing {
            Sizing::Fit => (
                u * width as f64,
                v * height as f64
            ),
            Sizing::Contain => {
                let scale = (width as f64 / res.0 as f64).max(height as f64 / res.1 as f64);
                (
                    (u - 0.5) * res.0 as f64 * scale + (width / 2) as f64,
                    (v - 0.5) * res.1 as f64 * scale + (height / 2) as f64
                )
            },
            Sizing::Cover => {
                let scale = (width as f64 / res.0 as f64).min(height as f64 / res.1 as f64);
                (
                    (u - 0.5) * res.0 as f64 * scale + (width / 2) as f64,
                    (v - 0.5) * res.1 as f64 * scale + (height / 2) as f64
                )
            },
            Sizing::DotByDot => (
                u * res.0 as f64,
                v * res.1 as f64
            )
        };
        if 0.0 <= x && x < width as f64 && 0.0 <= y && y < height as f64 {
            self.interpolation.interpolate(self.image.as_ref(), x, y)
        } else {
            self.default
        }
    }

    /*
    fn render(&self, ro: RenderOpt, buffer: &mut [T]) {
        let RenderOpt {u_res, v_res, frame_range, framerate, ..} = ro;
        for f in frame_range.start..frame_range.end {
            for v in 0..v_res {
                for u in 0..u_res {
                    buffer[(f - frame_range.start) as usize * u_res * v_res + v * u_res + u] =
                        self.sample(u as f64 / u_res as f64, v as f64 / v_res as f64, f as f64 / framerate as f64);
                }
            }
        }

   /}*/
}
