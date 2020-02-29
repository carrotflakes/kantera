use crate::render::{Res, Render, RenderOpt};
use crate::pixel::Rgba;

pub struct Sequencer<T: Copy, R: Render<T>> {
    pub clips: Vec<(f64, usize, R)>,
    pub default: T
}

impl<R: Render<Rgba>> Render<Rgba> for Sequencer<Rgba, R> {
    fn sample(&self, _u: f64, _v: f64, _time: f64, _res: Res) -> Rgba {
        panic!("sequencer not support sample");
    }

    fn render(&self, ro: &RenderOpt, buffer: &mut [Rgba]) {
        let RenderOpt {x_range, y_range, frame_range, framerate, ..} = ro;
        let frame_size = ((x_range.end - x_range.start) * (y_range.end - y_range.start)) as usize;

        for i in 0..buffer.len() {
            buffer[i] = self.default;
        }

        let mut inner_buffer = vec![self.default; buffer.len()];
        for (start, _, ref render) in self.clips.iter() {
            let start_frame = ro.frame_range.start - (start * *framerate as f64) as i32;
            let frame_range = start_frame.max(0)..((render.duration() * *framerate as f64) as i32).min(frame_range.end - (start * *framerate as f64) as i32);
            if frame_range.start >= frame_range.end {
                continue;
            }
            let ro = RenderOpt {
                frame_range: frame_range.clone(),
                ..ro.clone()
            };
            render.render(&ro, &mut inner_buffer[..(frame_range.end - frame_range.start) as usize * frame_size]);
            let offset = (frame_range.start - start_frame) as usize * frame_size;
            for i in 0..(frame_range.end - frame_range.start) as usize * frame_size {
                let j = i + offset;
                buffer[j] = buffer[j].normal_blend(&inner_buffer[i], 1.0);
            }
        }
    }
}

impl<T: Copy, R: Render<T>> Sequencer<T, R> {
    pub fn new(default: T) -> Self {
        Sequencer {
            clips: vec![],
            default
        }
    }

    pub fn append(mut self, time: f64, z: usize, render: R) -> Self {
        self.clips.push((time, z, render));
        self.clips.sort_by(|x, y| (x.1, x.0).partial_cmp(&(y.1, y.0)).unwrap());
        self
    }
}
