use crate::pixel::Rgba;
use crate::image::Image;
use crate::render::{Res, Render, RenderOpt};

pub struct Filter<R: Render<Rgba>> {
    pub render: R,
    pub filter: Image<Rgba>
}

impl<R: Render<Rgba>> Render<Rgba> for Filter<R> {
    fn sample(&self, _u: f64, _v: f64, _time: f64, _res: Res) -> Rgba {
        unimplemented!();
    }

    fn render(&self, ro: &RenderOpt, buffer: &mut [Rgba]) {
        let RenderOpt {x_range, y_range, res_x, res_y, frame_range, framerate, ..} = ro;
        let x_size = (ro.x_range.end - ro.x_range.start) as usize;
        let y_size = (ro.y_range.end - ro.y_range.start) as usize;
        let Image {width, height, ref vec} = self.filter;
        assert!(width % 2 == 1 && height % 2 == 1);
        let mut sub_buffer = vec![Rgba::default(); (res_x + width) * (res_y + height)];

        for f in frame_range.clone() {
            self.render.render(
                &RenderOpt {
                    x_range: x_range.start - (width / 2) as i32..x_range.end + (width / 2) as i32,
                    y_range: y_range.start - (height / 2) as i32..y_range.end + (height / 2) as i32,
                    res_x: *res_x,
                    res_y: *res_y,
                    frame_range: f..f + 1,
                    framerate: *framerate
                },
                sub_buffer.as_mut_slice());

            for y in 0..y_size {
                for x in 0..x_size {
                    let mut acc = Rgba(0.0, 0.0, 0.0, 0.0);

                    for fy in 0..height {
                        for fx in 0..width {
                            let p1 = sub_buffer[(y + fy) * (x_size + width - 1) + x + fx];
                            let p2 = vec[fy * width + fx];
                            acc.0 += p1.0 * p2.0;
                            acc.1 += p1.1 * p2.1;
                            acc.2 += p1.2 * p2.2;
                            acc.3 += p1.3 * p2.3;
                        }
                    }

                    buffer[(f - frame_range.start) as usize * x_size * y_size + y * x_size + x] = acc;
                }
            }
        }
    }

    fn duration(&self) -> f64 {
        self.render.duration()
    }
}

pub fn make_gaussian_filter(w: usize, h: usize, d: f64) -> Image<Rgba> {
    let dw = w * 2 + 1;
    let dh = h * 2 + 1;
    let mut vec = vec![Rgba::default(); dw * dh];
    let dd = 2.0 * d.powi(2);
    let ddpi = std::f64::consts::PI * dd;
    for y in 0..dh {
        let fy = y as f64 - h as f64;
        for x in 0..dw {
            let fx = x as f64 - w as f64;
            let v = (-(fx.powi(2) + fy.powi(2)) / dd).exp() / ddpi;
            vec[y * dw + x] = Rgba(v, v, v, v);
        }
    }
    Image {width: dw, height: dh, vec}
}
