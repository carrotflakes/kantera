use crate::audio_buffer::AudioBuffer;

#[derive(Debug)]
pub struct AudioRenderOpt {
    pub sample_range: std::ops::Range<i64>,
    pub sample_rate: usize,
}

pub trait AudioRender {
    fn render(&self, ro: &AudioRenderOpt) -> Vec<f64>;

    fn channel_num(&self) -> usize;
}

impl AudioRender for Box<dyn AudioRender> {
    #[inline(always)]
    fn render(&self, ro: &AudioRenderOpt) -> Vec<f64> {
        self.as_ref().render(ro)
    }
    
    fn channel_num(&self) -> usize {
        self.as_ref().channel_num()
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
}

pub struct Dummy();

impl AudioRender for Dummy {
    fn render(&self, ro: &AudioRenderOpt) -> Vec<f64> {
        let channel_num = self.channel_num();
        let size = (ro.sample_range.end - ro.sample_range.start) as usize;
        let mut buffer = Vec::with_capacity(channel_num * size);
        for _ in 0..channel_num {
            for i in ro.sample_range.clone() {
                let time = i as f64 / ro.sample_rate as f64;
                //buffer[c * size + (i - ro.sample_range.start) as usize] = 
                buffer.push((time * 440.0).sin());
            }
        }
        buffer
    }

    fn channel_num(&self) -> usize {
        2
    }
}

pub fn render_to_buffer(ro: &AudioRenderOpt,  render: &dyn AudioRender) -> AudioBuffer<f64> {
    // TODO: support step_sample_size for rendering large buffer
    let channel_num = render.channel_num();
    let size = (ro.sample_range.end - ro.sample_range.start) as usize;
    let raw_vec = render.render(ro);
    let mut vec = Vec::new();
    for c in 0..channel_num {
        vec.push(Vec::from(&raw_vec[c * size..(c + 1) * size]));
    }
    AudioBuffer {
        channel_num: channel_num,
        sample_num: (ro.sample_range.end - ro.sample_range.start) as usize,
        sample_rate: ro.sample_rate,
        vec: vec
    }
}

#[test]
fn test() {
    let buffer = render_to_buffer(
        &AudioRenderOpt {
            sample_range: 100..1000,
            sample_rate: 8000
        },
        &Dummy());
    assert_eq!(buffer.channel_num, 2);
    assert_eq!(buffer.sample_num, 1000 - 100);
    assert_eq!(buffer.sample_rate, 8000);
    assert_eq!(buffer.vec.len(), 2);
    assert_eq!(buffer.vec[0].len(), buffer.sample_num);
    assert_eq!(buffer.vec[1].len(), buffer.sample_num);
    assert_eq!(buffer.vec[0][0], (100.0f64 / 8000.0 * 440.0).sin());
    assert_eq!(buffer.vec[1][10], (110.0f64 / 8000.0 * 440.0).sin());
}
