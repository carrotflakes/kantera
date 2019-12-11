use crate::pixel::Rgba;
use crate::image::Image;
use crate::render::{Res, Range, Render, RenderOpt};

pub struct Filter {
    pub render: Box<dyn Render<Rgba>>,
    pub filter: Image<Rgba>
}

impl Render<Rgba> for Filter {
    fn sample(&self, _u: f64, _v: f64, _time: f64, _res: Res) -> Rgba {
        unimplemented!();
    }

    fn render(&self, ro: &RenderOpt, buffer: &mut [Rgba]) {
        let RenderOpt {u_range, u_res, v_range, v_res, frame_range, framerate, ..} = ro;
        let Image {width, height, ref vec} = self.filter;
        assert!(width % 2 == 1 && height % 2 == 1);
        let mut sub_buffer = vec![Rgba::default(); (u_res + width) * (v_res + height)];

        for f in frame_range.start..frame_range.end {
            self.render.render(
                &RenderOpt {
                    u_range: Range(u_range.0 - ((width / 2) as f64 / *u_res as f64),
                                   u_range.1 + ((width / 2) as f64 / *u_res as f64)),
                    u_res: u_res + width - 1,
                    v_range: Range(v_range.0 - ((height / 2) as f64 / *v_res as f64),
                                   v_range.1 + ((height / 2) as f64 / *v_res as f64)),
                    v_res: v_res + height - 1,
                    frame_range: f..f + 1,
                    framerate: *framerate
                },
                sub_buffer.as_mut_slice());

            for y in 0..*v_res {
                for x in 0..*u_res {
                    let mut acc = Rgba(0.0, 0.0, 0.0, 0.0);

                    for fy in 0..height {
                        for fx in 0..width {
                            let p1 = sub_buffer[(y + fy) * (u_res + width - 1) + x + fx];
                            let p2 = vec[fy * width + fx];
                            acc.0 += p1.0 * p2.0;
                            acc.1 += p1.1 * p2.1;
                            acc.2 += p1.2 * p2.2;
                            acc.3 += p1.3 * p2.3;
                        }
                    }

                    buffer[(f - frame_range.start) as usize * u_res * v_res + y * u_res + x] = acc;
                }
            }
        }
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
