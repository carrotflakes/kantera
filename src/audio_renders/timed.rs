use crate::audio_render::{AudioRenderOpt, AudioRender};
use crate::timed::Timed;

impl<T: Timed<f64>> AudioRender for T {
    fn render(&self, ro: &AudioRenderOpt) -> Vec<f64> {
        let size = (ro.sample_range.end - ro.sample_range.start) as usize;
        let mut vec = Vec::with_capacity(size);
        for i in 0..size {
            let time = (ro.sample_range.start as f64 + i as f64) / ro.sample_rate as f64;
            vec.push(self.get_value(time));
        }
        vec
    }

    fn channel_num(&self) -> usize {
        1
    }

    fn duration(&self) -> f64 {
        std::f64::INFINITY
    }
}
