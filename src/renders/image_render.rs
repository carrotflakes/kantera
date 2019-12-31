use std::rc::Rc;
use crate::render::{Res, Render};
use crate::image::Image;

#[derive(Debug, Copy, Clone)]
pub enum Sizing {
    Fit,
    Contain,
    Cover,
    DotByDot,
}

pub struct ImageRender<T: Copy> {
    pub image: Rc<Image<T>>,
    pub sizing: Sizing,
    pub default: T,
    //pub interpolation: Interpolation
}

impl <T: Copy> Render<T> for ImageRender<T> {
    fn sample(&self, u: f64, v: f64, _time: f64, res: Res) -> T {
        let width = self.image.width;
        let height = self.image.height;
        let (x, y) = match self.sizing {
            Sizing::Fit => (
                (u * width as f64).floor() as i32,
                (v * height as f64).floor() as i32
            ),
            Sizing::Contain => {
                let scale = (width as f64 / res.0 as f64).max(height as f64 / res.1 as f64);
                (
                    ((u - 0.5) * res.0 as f64 * scale).floor() as i32 + (width / 2) as i32,
                    ((v - 0.5) * res.1 as f64 * scale).floor() as i32 + (height / 2) as i32
                )
            },
            Sizing::Cover => {
                let scale = (width as f64 / res.0 as f64).min(height as f64 / res.1 as f64);
                (
                    ((u - 0.5) * res.0 as f64 * scale).floor() as i32 + (width / 2) as i32,
                    ((v - 0.5) * res.1 as f64 * scale).floor() as i32 + (height / 2) as i32
                )
            },
            Sizing::DotByDot => (
                (u * res.0 as f64).floor() as i32,
                (v * res.1 as f64).floor() as i32
            )
        };
        if (0..width as i32).contains(&x) && (0..height as i32).contains(&y) {
            self.image.vec[y as usize * width + x as usize].clone()
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
