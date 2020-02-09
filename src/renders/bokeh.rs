use crate::pixel::Rgba;
use crate::render::{Res, Render, RenderOpt};
use crate::path::Timed;
use std::rc::Rc;

pub struct Bokeh<R: Render<Rgba>> {
    pub render: R,
    pub max_size: usize,
    pub size: Rc<dyn Timed<f64>> // TODO: make pixel size
}

impl<R: Render<Rgba>> Render<Rgba> for Bokeh<R> {
    fn sample(&self, _u: f64, _v: f64, _time: f64, _res: Res) -> Rgba {
        panic!("Bokeh cannot sample");
    }

    fn render(&self, ro: &RenderOpt, buffer: &mut [Rgba]) {
        let RenderOpt {x_range, y_range, res_x,  res_y, frame_range, framerate, ..} = ro;
        let x_size = (ro.x_range.end - ro.x_range.start) as usize;
        let y_size = (ro.y_range.end - ro.y_range.start) as usize;
        let size = self.max_size;
        let mut sub_buffer = vec![Rgba::default(); (res_x + size * 2 + 1) * (res_y + size * 2 + 1)];

        for f in frame_range.start..frame_range.end {
            let time = f as f64 / *framerate as f64;
            self.render.render(
                &RenderOpt {
                    x_range: x_range.start - size as i32..x_range.end + size as i32,
                    y_range: y_range.start - size as i32..y_range.end + size as i32,
                    res_x: *res_x,
                    res_y: *res_y,
                    frame_range: f..f + 1,
                    framerate: *framerate
                },
                sub_buffer.as_mut_slice());

            let real_size: usize = (self.size.get_value(time).round().abs() as usize).min(size);

            for y in 0..y_size + size * 2 {
                let mut acc = Rgba(0.0, 0.0, 0.0, 0.0);
                for x in size - real_size..size + real_size + 1 {
                    let rgba = &sub_buffer[y * (x_size + size * 2) + x];
                    acc.0 += rgba.0;
                    acc.1 += rgba.1;
                    acc.2 += rgba.2;
                    acc.3 += rgba.3;
                }
                for x in size..x_size + size {
                    let left = sub_buffer[y * (x_size + size * 2) + x - real_size];

                    let rgba = &mut sub_buffer[y * (x_size + size * 2) + x - size];
                    rgba.0 = acc.0 / (real_size * 2 + 1) as f64;
                    rgba.1 = acc.1 / (real_size * 2 + 1) as f64;
                    rgba.2 = acc.2 / (real_size * 2 + 1) as f64;
                    rgba.3 = acc.3 / (real_size * 2 + 1) as f64;

                    let right = &sub_buffer[y * (x_size + size * 2) + x + real_size + 1];
                    acc.0 += right.0 - left.0;
                    acc.1 += right.1 - left.1;
                    acc.2 += right.2 - left.2;
                    acc.3 += right.3 - left.3;
                }
            }

            for x in 0..x_size + size * 2 { // TOOD
                let mut acc = Rgba(0.0, 0.0, 0.0, 0.0);
                for y in size - real_size..size + real_size + 1 {
                    let rgba = &sub_buffer[y * (x_size + size * 2) + x];
                    acc.0 += rgba.0;
                    acc.1 += rgba.1;
                    acc.2 += rgba.2;
                    acc.3 += rgba.3;
                }
                for y in size..res_y + size {
                    let left = sub_buffer[(y - real_size) * (x_size + size * 2) + x];

                    let rgba = &mut sub_buffer[(y - size) * (x_size + size * 2) + x];
                    rgba.0 = acc.0 / (real_size * 2 + 1) as f64;
                    rgba.1 = acc.1 / (real_size * 2 + 1) as f64;
                    rgba.2 = acc.2 / (real_size * 2 + 1) as f64;
                    rgba.3 = acc.3 / (real_size * 2 + 1) as f64;

                    let right = &sub_buffer[(y + real_size + 1) * (x_size + size * 2) + x];
                    acc.0 += right.0 - left.0;
                    acc.1 += right.1 - left.1;
                    acc.2 += right.2 - left.2;
                    acc.3 += right.3 - left.3;
                }
            }

            for y in 0..y_size {
                for x in 0..x_size {
                    buffer[(f - frame_range.start) as usize * x_size * y_size + y * x_size + x] =
                        sub_buffer[y * (x_size + size * 2) + x]
                }
            }
        }
    }
}
