use crate::audio_render::{AudioRenderOpt, AudioRender};

pub struct Sequencer<R: AudioRender> {
    pub renders: Vec<(f64, R)>
}

impl<R: AudioRender> AudioRender for Sequencer<R> {
    fn render(&self, ro: &AudioRenderOpt) -> Vec<f64> {
        let channel_num = self.channel_num();
        let size = (ro.sample_range.end - ro.sample_range.start) as usize;
        let mut vec = vec![0.0; channel_num * size];
        let start = ro.sample_range.start as f64 * ro.sample_rate as f64;
        let end = ro.sample_range.end as f64 * ro.sample_rate as f64;
        for (time, render) in self.renders.iter() {
            let ro_start = (0.0f64.max(start - time) * ro.sample_rate as f64).floor() as usize;
            let ro_end = (render.duration().min(end - time) * ro.sample_rate as f64).floor() as usize;
            if ro_end <= ro_start {
                continue;
            }
            let ro = AudioRenderOpt {
                sample_range: ro_start as i64..ro_end as i64,
                sample_rate: ro.sample_rate
            };
            let rendered_vec = render.render(&ro);
            for c in 0..channel_num {
                for i in 0..ro_end - ro_start {
                    vec[c * size + ((time * ro.sample_rate as f64) as usize - ro_start - ro.sample_range.start as usize) + i]
                        += rendered_vec[c * (ro_end - ro_start) + i];
                }
            }
        }
        vec
    }

    fn channel_num(&self) -> usize {
        self.renders[0].1.channel_num()
    }

    fn duration(&self) -> f64 {
        self.renders.iter().fold(0.0, |acc, x| acc.max(x.0 + x.1.duration()))
    }
}

impl<R: AudioRender> Sequencer<R> {
    pub fn new() -> Self {
        Sequencer {
            renders: vec![]
        }
    }

    pub fn append(mut self, time: f64, render: R) -> Self {
        self.renders.push((time, render));
        self
    }
}
