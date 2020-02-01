#[derive(Debug)]
pub struct AudioBuffer<T> {
    pub channel_num: usize,
    pub sample_num: usize,
    pub sample_rate: usize,
    pub vec: Vec<Vec<T>>
}

pub fn make_audio(sec: f64) -> AudioBuffer<u16> {
    let mut l = Vec::new();
    let mut r = Vec::new();
    for i in 0..(44100.0 * sec) as usize {
        let p = i as f64 / 44100.0 * 440.0 * std::f64::consts::PI * 2.0;
        l.push(((p.sin() + 1.0) / 2.0 * std::u16::MAX as f64) as u16);
        r.push((((p * 2.0).sin() + 1.0) / 2.0 * std::u16::MAX as f64) as u16);
    }
    l.shrink_to_fit();
    r.shrink_to_fit();
    AudioBuffer {
        channel_num: 2,
        sample_num: l.len(),
        sample_rate: 44100,
        vec: vec![l, r],
    }
}

impl From<&AudioBuffer<u16>> for AudioBuffer<f32> {
    fn from(ab: &AudioBuffer<u16>) -> AudioBuffer<f32> {
        let mut vec = vec![];
        for v in ab.vec.iter() {
            vec.push(v.iter().map(|x| *x as f32 / std::u16::MAX as f32 * 2.0 - 1.0).collect());
        }
        AudioBuffer {
            channel_num: ab.channel_num,
            sample_num: ab.sample_num,
            sample_rate: ab.sample_rate,
            vec
        }
    }
}

impl From<&AudioBuffer<f64>> for AudioBuffer<u16> {
    fn from(ab: &AudioBuffer<f64>) -> AudioBuffer<u16> {
        let mut vec = vec![];
        for v in ab.vec.iter() {
            vec.push(v.iter().map(|x| ((*x + 1.0) / 2.0 * std::u16::MAX as f64).round() as u16).collect());
        }
        AudioBuffer {
            channel_num: ab.channel_num,
            sample_num: ab.sample_num,
            sample_rate: ab.sample_rate,
            vec
        }
    }
}
