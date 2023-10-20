use std::rc::Rc;
use crate::audio_render::{AudioRenderOpt, AudioRender};
use crate::audio_buffer::AudioBuffer;
use crate::interpolation::AudioInterpolation;

pub struct AudioBufferRender<T, U: AudioInterpolation<T>> {
    pub audio_buffer: Rc<AudioBuffer<T>>,
    pub interpolation: U
}

impl<U: AudioInterpolation<f64>> AudioRender for AudioBufferRender<f64, U> {
    fn render(&self, ro: &AudioRenderOpt) -> Vec<f64> {
        let channel_num = self.channel_num();
        let size = (ro.sample_range.end - ro.sample_range.start) as usize;
        let mut vec = vec![0.0; channel_num * size];
        let r = self.audio_buffer.sample_rate as f64 / ro.sample_rate as f64;
        let a = (self.audio_buffer.sample_num as f64 / r - ro.sample_range.start as f64).max(0.0) as usize;
        for c in 0..channel_num {
            for i in 0..size.min(a) {
                let x = (i as i64 + ro.sample_range.start) as f64 * r;
                vec[c * size + i] = self.interpolation.interpolate(&self.audio_buffer.vec[c], x);
            }
        }
        vec
    }

    fn channel_num(&self) -> usize {
        self.audio_buffer.channel_num
    }

    fn duration(&self) -> f64 {
        self.audio_buffer.sample_num as f64 / self.audio_buffer.sample_rate as f64
    }
}

impl<U: AudioInterpolation<u16>> AudioRender for AudioBufferRender<u16, U> {
    fn render(&self, ro: &AudioRenderOpt) -> Vec<f64> {
        let channel_num = self.channel_num();
        let size = (ro.sample_range.end - ro.sample_range.start) as usize;
        let mut vec = vec![0.0; channel_num * size];
        let r = self.audio_buffer.sample_rate as f64 / ro.sample_rate as f64;
        let a = (self.audio_buffer.sample_num as f64 / r - ro.sample_range.start as f64).max(0.0) as usize;
        for c in 0..channel_num {
            for i in 0..size.min(a) {
                let x = (i as i64 + ro.sample_range.start) as f64 * r;
                vec[c * size + i] = self.interpolation.interpolate(&self.audio_buffer.vec[c], x) as f64 * 2.0 / std::u16::MAX as f64 - 1.0;
            }
        }
        vec
    }

    fn channel_num(&self) -> usize {
        self.audio_buffer.channel_num
    }

    fn duration(&self) -> f64 {
        self.audio_buffer.sample_num as f64 / self.audio_buffer.sample_rate as f64
    }
}
