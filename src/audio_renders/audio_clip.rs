use crate::audio_buffer::{pan, pan_mono};
use crate::audio_render::{AudioRenderOpt, AudioRender};

pub struct AudioClip<T: AudioRender> {
    pub audio_render: T,
    pub gain: f64,
    pub pan: f64,
    pub start: f64,
    pub duration: f64,
    pub pitch: f64,
    pub fadein: f64,
    pub fadeout: f64
}

impl<T: AudioRender> AudioRender for AudioClip<T> {
    fn render(&self, ro: &AudioRenderOpt) -> Vec<f64> {
        let channel_num = self.channel_num();
        let size = (ro.sample_range.end - ro.sample_range.start) as usize;
        let start = ro.sample_range.start as f64 / ro.sample_rate as f64;
        let end = ro.sample_range.end as f64 / ro.sample_rate as f64;
        let rendered_vec = {
            let sample_rate = (ro.sample_rate as f64 / self.pitch) as usize;
            let local_start = self.start + start * self.pitch;
            let local_end = self.start + end * self.pitch;
            let sample_range_start = local_start * sample_rate as f64;
            let sample_range_end = local_end * sample_rate as f64;
            assert_eq!(size, (sample_range_end - sample_range_start) as usize);
            self.audio_render.render(&AudioRenderOpt {
                sample_range: sample_range_start as i64..sample_range_end as i64,
                sample_rate: sample_rate
            })
        };
        let mut vec = vec![0.0; channel_num * size];
        match self.audio_render.channel_num() {
            1 =>
                for i in 0..size {
                    let (l, r) = pan_mono(rendered_vec[i], self.pan);
                    vec[i] = l * self.gain;
                    vec[size + i] = r * self.gain;
                }
            2 =>
                for i in 0..size {
                    let (l, r) = pan(rendered_vec[i], rendered_vec[size + i], self.pan);
                    vec[i] = l * self.gain;
                    vec[size + i] = r * self.gain;
                }
            channel_num => panic!("AudioClip not support channel_num: {:?}", channel_num)
        }
        // fadein
        {
            let fade_duration = self.fadein * ro.sample_rate as f64;
            let len = fade_duration as i64 - ro.sample_range.start;
            if 0 < len {
                println!("{:?}", len);
                for c in 0..channel_num {
                    for i in 0..size.min(len as usize) {
                        let scale = (i + ro.sample_range.start as usize) as f64 / fade_duration; // TOOD: valid power?
                        let scale = 10.0f64.powf((scale - 1.0) * 2.0);
                        vec[c * size + i] *= scale;
                    }
                }
            }
        }
        // fadeout
        {
            let fade_duration = self.fadeout * ro.sample_rate as f64;
            let total_len = (self.duration / self.pitch * ro.sample_rate as f64) as i64 - ro.sample_range.end;
            let len = fade_duration as i64 - total_len;
            if 0 < len {
                println!("{:?}", len);
                for c in 0..channel_num {
                    for i in 0..size.min(len as usize) {
                        let scale = (i + total_len as usize) as f64 / fade_duration; // TOOD: valid power?
                        let scale = 10.0f64.powf((scale - 1.0) * 2.0);
                        vec[(c + 1) * size - i - 1] *= scale;
                    }
                }
            }
        }
        vec
    }

    fn channel_num(&self) -> usize {
        2
    }

    fn duration(&self) -> f64 {
        self.duration / self.pitch
    }
}
