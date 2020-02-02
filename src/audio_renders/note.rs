use crate::audio_render::{AudioRenderOpt, AudioRender};

pub struct Note {
    pub frequency: f64,
    pub gain: f64,
    pub duration: f64,
    pub pan: f64
}

impl AudioRender for Note {
    fn render(&self, ro: &AudioRenderOpt) -> Vec<f64> {
        let channel_num = self.channel_num();
        let size = (ro.sample_range.end - ro.sample_range.start) as usize;
        let mut vec = vec![0.0; channel_num * size];
        for i in 0..size {
            let phase = (i as i64 + ro.sample_range.start) as f64 / ro.sample_rate as f64 * self.frequency * std::f64::consts::PI * 2.0;
            let v = phase.sin() * self.gain;
            vec[i] = v * (1.0 - self.pan) / 2.0;
            vec[size + i] = v * (self.pan + 1.0) / 2.0;
        }
        vec
    }

    fn channel_num(&self) -> usize {
        2
    }

    fn duration(&self) -> f64 {
        self.duration
    }
}
