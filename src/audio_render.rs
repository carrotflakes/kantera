use crate::audio_buffer::AudioBuffer;

#[derive(Debug)]
pub struct AudioRenderOpt {
    pub sample_range: std::ops::Range<i64>,
    pub sample_rate: usize,
}

pub trait AudioRender {
    fn render(&self, ro: &AudioRenderOpt) -> Vec<f64>;

    fn channel_num(&self) -> usize;

    fn duration(&self) -> f64;
}

impl AudioRender for Box<dyn AudioRender> {
    #[inline(always)]
    fn render(&self, ro: &AudioRenderOpt) -> Vec<f64> {
        self.as_ref().render(ro)
    }

    fn channel_num(&self) -> usize {
        self.as_ref().channel_num()
    }

    fn duration(&self) -> f64 {
        self.as_ref().duration()
    }
}
impl AudioRender for std::rc::Rc<dyn AudioRender> {
    #[inline(always)]
    fn render(&self, ro: &AudioRenderOpt) -> Vec<f64> {
        self.as_ref().render(ro)
    }

    fn channel_num(&self) -> usize {
        self.as_ref().channel_num()
    }

    fn duration(&self) -> f64 {
        self.as_ref().duration()
    }
}

pub struct Dummy(pub f64);

impl AudioRender for Dummy {
    fn render(&self, ro: &AudioRenderOpt) -> Vec<f64> {
        let channel_num = self.channel_num();
        let size = (ro.sample_range.end - ro.sample_range.start) as usize;
        let mut buffer = Vec::with_capacity(channel_num * size);
        for _ in 0..channel_num {
            for i in ro.sample_range.clone() {
                let time = i as f64 / ro.sample_rate as f64;
                //buffer[c * size + (i - ro.sample_range.start) as usize] =
                buffer.push((time * 440.0 * std::f64::consts::PI * 2.0).sin() * 0.1);
            }
        }
        buffer
    }

    fn channel_num(&self) -> usize {
        2
    }

    fn duration(&self) -> f64 {
        self.0
    }
}

pub fn render_to_buffer(render: &dyn AudioRender, sample_rate: usize) -> AudioBuffer<f64> {
    // TODO: support step_sample_size for rendering large buffer
    let channel_num = render.channel_num();
    let size = (render.duration() * sample_rate as f64).floor() as usize;
    let raw_vec = render.render(&AudioRenderOpt {
        sample_range: 0..size as i64,
        sample_rate: sample_rate
    });
    let mut vec = Vec::new();
    for c in 0..channel_num {
        vec.push(Vec::from(&raw_vec[c * size..(c + 1) * size]));
    }
    AudioBuffer {
        channel_num: channel_num,
        sample_num: size,
        sample_rate: sample_rate,
        vec: vec
    }
}

#[test]
fn test() {
    let buffer = render_to_buffer(&Dummy(100.0), 8000);
    assert_eq!(buffer.channel_num, 2);
    assert_eq!(buffer.sample_num, 8000 * 100);
    assert_eq!(buffer.sample_rate, 8000);
    assert_eq!(buffer.vec.len(), 2);
    assert_eq!(buffer.vec[0].len(), buffer.sample_num);
    assert_eq!(buffer.vec[1].len(), buffer.sample_num);
    assert_eq!(buffer.vec[0][0], (0.0f64 / 8000.0 * 440.0 * std::f64::consts::PI * 2.0).sin() * 0.1);
    assert_eq!(buffer.vec[1][10], (10.0f64 / 8000.0 * 440.0 * std::f64::consts::PI * 2.0).sin() * 0.1);
}
