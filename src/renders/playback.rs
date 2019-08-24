use crate::render::{Render, RenderOpt};
use crate::buffer::Buffer;

pub enum Interpolation {
    NearestNeighbor,
    Linear
}

pub struct Playback<T> {
    pub buffer: Box<Buffer<T>>,
//    pub interpolation: Interpolation
}

impl <T: Default + Clone> Render<T> for Playback<T> {
    fn sample(&self, u: f64, v: f64, time: f64) -> T {
        //let Buffer {width, height, frame_num, framerate, vec} = self.buffer;
        let frame_num = self.buffer.frame_num;
        let framerate = self.buffer.framerate;
        let width = self.buffer.width;
        let height = self.buffer.height;
        let t = (time * framerate as f64).floor() as usize;
        let x = (u * width as f64).floor() as usize;
        let y = (v * height as f64).floor() as usize;
        if (0..width).contains(&x) &&
            (0..height).contains(&y) &&
            (0..frame_num).contains(&t) {
                self.buffer.vec[t * width * height + y * width + x].clone()
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

impl<T: Default + Clone> From<Buffer<T>> for Playback<T> {
    fn from(buffer: Buffer<T>) -> Playback<T> {
        Playback {buffer: Box::new(buffer)}
    }
}
