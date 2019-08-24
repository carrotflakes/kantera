use crate::pixel::Rgba;
use crate::render::{Render, RenderOpt};
use crate::path::Path;

pub struct Bokeh {
    pub render: Box<Render<Rgba>>,
    pub max_size: usize,
    pub size_path: Path
}

impl Render<Rgba> for Bokeh {
    fn sample(&self, u: f64, v: f64, time: f64) -> Rgba {
        panic!("Bokeh cannot sample");
    }

    fn render(&self, ro: &RenderOpt, buffer: &mut [Rgba]) {
        let RenderOpt {u_range, u_res, v_range, v_res, frame_range, framerate, ..} = ro;
        let size = self.max_size;
        let frame_size = u_res * v_res;
        let mut sub_buffer = vec![Rgba::default(); (u_res + size * 2 + 1) * (v_res + size * 2 + 1)];

        for f in frame_range.start..frame_range.end {
            let time = f as f64 / *framerate as f64;
            self.render.render(
                &RenderOpt {
                    u_range: u_range.start - (size as f64 / *u_res as f64)..
                        u_range.end + (size as f64 / *u_res as f64),
                    u_res: u_res + size * 2,
                    v_range: v_range.start - (size as f64 / *v_res as f64)..
                        v_range.end + (size as f64 / *v_res as f64),
                    v_res: v_res + size * 2,
                    frame_range: f..f + 1,
                    framerate: *framerate
                },
                sub_buffer.as_mut_slice());

            let real_size: usize = (self.size_path.get_value(time).round().abs() as usize).min(size);

            for y in 0..*v_res + size * 2 {
                let mut acc = Rgba(0.0, 0.0, 0.0, 0.0);
                for x in size - real_size..size + real_size + 1 {
                    let rgba = &sub_buffer[y * (u_res + size * 2) + x];
                    acc.0 += rgba.0;
                    acc.1 += rgba.1;
                    acc.2 += rgba.2;
                    acc.3 += rgba.3;
                }
                for x in size..*u_res + size {
                    let rgba = &mut sub_buffer[y * (u_res + size * 2) + x - size];
                    rgba.0 = acc.0 / (real_size * 2 + 1) as f64;
                    rgba.1 = acc.1 / (real_size * 2 + 1) as f64;
                    rgba.2 = acc.2 / (real_size * 2 + 1) as f64;
                    rgba.3 = acc.3 / (real_size * 2 + 1) as f64;

                    let left = &sub_buffer[y * (u_res + size * 2) + x - real_size];
                    let right = &sub_buffer[y * (u_res + size * 2) + x + real_size + 1];
                    acc.0 += right.0 - left.0;
                    acc.1 += right.1 - left.1;
                    acc.2 += right.2 - left.2;
                    acc.3 += right.3 - left.3;
                }
            }

            for x in 0..*u_res + size * 2 { // TOOD
                let mut acc = Rgba(0.0, 0.0, 0.0, 0.0);
                for y in size - real_size..size + real_size + 1 {
                    let rgba = &sub_buffer[y * (u_res + size * 2) + x];
                    acc.0 += rgba.0;
                    acc.1 += rgba.1;
                    acc.2 += rgba.2;
                    acc.3 += rgba.3;
                }
                for y in size..*v_res + size {
                    let rgba = &mut sub_buffer[(y - size) * (u_res + size * 2) + x];
                    rgba.0 = acc.0 / (real_size * 2 + 1) as f64;
                    rgba.1 = acc.1 / (real_size * 2 + 1) as f64;
                    rgba.2 = acc.2 / (real_size * 2 + 1) as f64;
                    rgba.3 = acc.3 / (real_size * 2 + 1) as f64;

                    let left = &sub_buffer[(y - real_size) * (u_res + size * 2) + x];
                    let right = &sub_buffer[(y + real_size + 1) * (u_res + size * 2) + x];
                    acc.0 += right.0 - left.0;
                    acc.1 += right.1 - left.1;
                    acc.2 += right.2 - left.2;
                    acc.3 += right.3 - left.3;
                }
            }

            for y in 0..*v_res {
                for x in 0..*u_res {
                    buffer[(f - frame_range.start) as usize * u_res * v_res + y * u_res + x] =
                        sub_buffer[y * (u_res + size * 2) + x]
                }
            }
        }
    }
}
