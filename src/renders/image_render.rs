use crate::render::{Rgba, Render, RenderOpt};
use crate::image::Image;

pub enum Size {
    Auto,
    Contain,
    Cover,
}

pub struct ImageRender<T> {
    pub image: Box<Image<T>>,
//    pub interpolation: Interpolation
}

impl <T: Default + Clone> Render<T> for ImageRender<T> {
    fn sample(&self, u: f64, v: f64, time: f64) -> T {
        if (0.0..=1.0).contains(&u) && (0.0..=1.0).contains(&v) {
            let width = self.image.width;
            let height = self.image.height;
            let x = (u * width as f64).floor() as usize;
            let y = (v * height as f64).floor() as usize;
            self.image.vec[y * width + x].clone()
        } else {
            T::default()
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
