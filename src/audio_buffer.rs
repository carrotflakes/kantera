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
        r.push(((p.sin() + 1.0) / 2.0 * std::u16::MAX as f64) as u16);
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
