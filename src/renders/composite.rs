use crate::pixel::Rgba;
use crate::render::{Res, Render, RenderOpt};
use crate::path::Timed;
use std::rc::Rc;

pub enum CompositeMode {
    None,
    Normal(Rc<dyn Timed<f64>>)
}

pub struct Composite<R: Render<Rgba>> {
    pub layers: Vec<(R, CompositeMode)>
}

impl<R: Render<Rgba>> Render<Rgba> for Composite<R> {
    fn sample(&self, u: f64, v: f64, time: f64, res: Res) -> Rgba {
        let mut values = [Rgba::default()];
        for (render, cm) in &self.layers {
            composite(&mut values, &[render.sample(u, v, time, res)], time, cm);
        }
        values[0]
    }

    fn render(&self, ro: &RenderOpt, buffer: &mut [Rgba]) {
        let RenderOpt {x_range, y_range, frame_range, framerate, ..} = ro;
        let mut sub_buffer = vec![Rgba::default(); buffer.len()];
        let frame_size = ((x_range.end - x_range.start) * (y_range.end - y_range.start)) as usize;

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

    fn duration(&self) -> f64 {
        self.layers.iter().map(|x| x.0.duration()).fold(std::f64::INFINITY, |x, y| x.min(y))
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
