use crate::render::{Rgba, Render, RenderOpt};
use crate::path::Path;

#[derive(Debug)]
pub enum CompositeMode {
    None,
    Normal(Path)
}

pub struct Composite {
    pub layers: Vec<(Box<Render<Rgba>>, CompositeMode)>
}

impl Render<Rgba> for Composite {
    fn sample(&self, u: f64, v: f64, time: f64) -> Rgba {
        let mut value = Rgba::default();
        for (render, cm) in &self.layers {
            composite(&mut value, &render.sample(u, v, time), time, cm);
        }
        value
    }

    fn render(&self, ro: &RenderOpt, buffer: &mut [Rgba]) {
        let RenderOpt {u_range, u_res, v_range, v_res, frame_range, framerate, ..} = ro;
        let mut sub_buffer = vec![Rgba::default(); buffer.len()];

        for (render, cm) in &self.layers {
            render.render(ro, sub_buffer.as_mut_slice());
            for f in frame_range.start..frame_range.end {
                let time = f as f64 / *framerate as f64;
                for xy in 0..(*v_res * *u_res) {
                    let i = (f - frame_range.start) as usize * u_res * v_res + xy;
                        composite(&mut buffer[i], &sub_buffer[i], time, cm);
                }
            }
        }
    }
}

#[inline(always)]
fn composite(base: &mut Rgba, value: &Rgba, time: f64, cm: &CompositeMode) {
    *base = match cm {
        CompositeMode::None => *value,
        CompositeMode::Normal(alpha_path) =>
            base.normal_blend(value, alpha_path.get_value(time))
    };
}
