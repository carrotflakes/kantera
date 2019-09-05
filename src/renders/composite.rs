use crate::pixel::Rgba;
use crate::render::{Res, Render, RenderOpt};
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
    fn sample(&self, u: f64, v: f64, time: f64, res: Res) -> Rgba {
        let mut value = Rgba::default();
        for (render, cm) in &self.layers {
            composite(&mut [value], &[render.sample(u, v, time, res)], time, cm);
        }
        value
    }

    fn render(&self, ro: &RenderOpt, buffer: &mut [Rgba]) {
        let RenderOpt {u_res, v_res, frame_range, framerate, ..} = ro;
        let mut sub_buffer = vec![Rgba::default(); buffer.len()];
        let frame_size = u_res * v_res;

        for (render, cm) in &self.layers {
            render.render(ro, sub_buffer.as_mut_slice());
            for f in frame_range.start..frame_range.end {
                let start = frame_size * (f - frame_range.start) as usize;
                composite(
                    &mut buffer[start..start+frame_size],
                    &sub_buffer[start..start+frame_size],
                    f as f64 / *framerate as f64, cm);
            }
        }
    }
}

#[inline(always)]
fn composite(base: &mut [Rgba], value: &[Rgba], time: f64, cm: &CompositeMode) {
    match cm {
        CompositeMode::None => {
            for i in 0..base.len() {
                base[i] = value[i];
            }
        },
        CompositeMode::Normal(alpha_path) => {
            let alpha = alpha_path.get_value(time);
            for i in 0..base.len() {
                base[i] = base[i].normal_blend(&value[i], alpha);
            }
        }
    };
}
