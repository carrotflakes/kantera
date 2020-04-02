use crate::pixel::Rgba;
use crate::render::{Res, Render, RenderOpt};

pub enum ColorSamplingType {
    T444, // 1x1
    T422, // 2x1
    T420, // 2x2
    T411, // 4x1
}

pub struct ColorSampling<R: Render<Rgba>> {
    pub render: R,
    pub r#type: ColorSamplingType
}

impl <R: Render<Rgba>> Render<Rgba> for ColorSampling<R> {
    fn sample(&self, _u: f64, _v: f64, _time: f64, _res: Res) -> Rgba {
        unimplemented!();
    }

    fn render(&self, ro: &RenderOpt, buffer: &mut [Rgba]) {
        self.render.render(ro, buffer);

        let RenderOpt {res_x, res_y, frame_range, x_range, y_range, ..} = ro;
        let frame_size = *res_x * *res_y;
        let x_size = (x_range.end - x_range.start) as usize;
        let y_size = (y_range.end - y_range.start) as usize;

        use ColorSamplingType::*;
        match self.r#type {
            T444 => {
                for f in 0..(frame_range.end - frame_range.start) as usize {
                    for y in 0..y_size {
                        for x in 0..x_size {
                            let p = buffer[f * frame_size + y * x_size + x];
                            let q = rgb_to_ypbpr(p.0, p.1, p.2);
                            let (r, g, b) = ypbpr_to_rgb(q.0, q.1, q.2);
                            buffer[f * frame_size + y * x_size + x] = Rgba(r, g, b, 1.0);
                        }
                    }
                }
            }
            T422 => {
                for f in 0..(frame_range.end - frame_range.start) as usize {
                    for y in 0..y_size {
                        for x in 0..x_size / 2 {
                            let p1 = buffer[f * frame_size + y * x_size + x * 2];
                            let p2 = buffer[f * frame_size + y * x_size + x * 2 + 1];
                            let q1 = rgb_to_ypbpr(p1.0, p1.1, p1.2);
                            let q2 = rgb_to_ypbpr(p2.0, p2.1, p2.2);
                            let rgb1 = ypbpr_to_rgb(q1.0, q1.1, q1.2);
                            let rgb2 = ypbpr_to_rgb(q2.0, q1.1, q1.2);
                            buffer[f * frame_size + y * x_size + x * 2] = Rgba(rgb1.0, rgb1.1, rgb1.2, 1.0);
                            buffer[f * frame_size + y * x_size + x * 2 + 1] = Rgba(rgb2.0, rgb2.1, rgb2.2, 1.0);
                        }
                    }
                }
            }
            T420 => {
                for f in 0..(frame_range.end - frame_range.start) as usize {
                    for y in 0..y_size / 2 {
                        for x in 0..x_size / 2 {
                            let p1 = buffer[f * frame_size + y * 2 * x_size + x * 2];
                            let p2 = buffer[f * frame_size + y * 2 * x_size + x * 2 + 1];
                            let p3 = buffer[f * frame_size + (y * 2 + 1) * x_size + x * 2];
                            let p4 = buffer[f * frame_size + (y * 2 + 1) * x_size + x * 2 + 1];
                            let q1 = rgb_to_ypbpr(p1.0, p1.1, p1.2);
                            let q2 = rgb_to_ypbpr(p2.0, p2.1, p2.2);
                            let q3 = rgb_to_ypbpr(p3.0, p3.1, p3.2);
                            let q4 = rgb_to_ypbpr(p4.0, p4.1, p4.2);
                            let rgb1 = ypbpr_to_rgb(q1.0, q1.1, q3.2);
                            let rgb2 = ypbpr_to_rgb(q2.0, q1.1, q3.2);
                            let rgb3 = ypbpr_to_rgb(q3.0, q1.1, q3.2);
                            let rgb4 = ypbpr_to_rgb(q4.0, q1.1, q3.2);
                            buffer[f * frame_size + y * 2 * x_size + x * 2] = Rgba(rgb1.0, rgb1.1, rgb1.2, 1.0);
                            buffer[f * frame_size + y * 2 * x_size + x * 2 + 1] = Rgba(rgb2.0, rgb2.1, rgb2.2, 1.0);
                            buffer[f * frame_size + (y * 2 + 1) * x_size + x * 2 + 1] = Rgba(rgb3.0, rgb3.1, rgb3.2, 1.0);
                            buffer[f * frame_size + (y * 2 + 1) * x_size + x * 2 + 1] = Rgba(rgb4.0, rgb4.1, rgb4.2, 1.0);
                        }
                    }
                }
            }
            T411 => {
                for f in 0..(frame_range.end - frame_range.start) as usize {
                    for y in 0..y_size {
                        for x in 0..x_size / 4 {
                            let p1 = buffer[f * frame_size + y * x_size + x * 4];
                            let p2 = buffer[f * frame_size + y * x_size + x * 4 + 1];
                            let p3 = buffer[f * frame_size + y * x_size + x * 4 + 2];
                            let p4 = buffer[f * frame_size + y * x_size + x * 4 + 3];
                            let q1 = rgb_to_ypbpr(p1.0, p1.1, p1.2);
                            let q2 = rgb_to_ypbpr(p2.0, p2.1, p2.2);
                            let q3 = rgb_to_ypbpr(p3.0, p3.1, p3.2);
                            let q4 = rgb_to_ypbpr(p4.0, p4.1, p4.2);
                            let rgb1 = ypbpr_to_rgb(q1.0, q1.1, q1.2);
                            let rgb2 = ypbpr_to_rgb(q2.0, q1.1, q1.2);
                            let rgb3 = ypbpr_to_rgb(q3.0, q1.1, q1.2);
                            let rgb4 = ypbpr_to_rgb(q4.0, q1.1, q1.2);
                            buffer[f * frame_size + y * x_size + x * 4] = Rgba(rgb1.0, rgb1.1, rgb1.2, 1.0);
                            buffer[f * frame_size + y * x_size + x * 4 + 1] = Rgba(rgb2.0, rgb2.1, rgb2.2, 1.0);
                            buffer[f * frame_size + y * x_size + x * 4 + 2] = Rgba(rgb3.0, rgb3.1, rgb3.2, 1.0);
                            buffer[f * frame_size + y * x_size + x * 4 + 3] = Rgba(rgb4.0, rgb4.1, rgb4.2, 1.0);
                        }
                    }
                }
            }
        }
    }

    fn duration(&self) -> f64 {
        self.render.duration()
    }
}

fn rgb_to_ypbpr(r: f64, g: f64, b: f64) -> (f64, f64, f64) {
    let y = 0.299 * r + 0.587 * g + 0.114 * b;
    let pb = 0.5 * (b - y) / (1.0 - 0.114);
    let pr = 0.5 * (r - y) / (1.0 - 0.299);
    (y, pb, pr)
}

fn ypbpr_to_rgb(y: f64, pb: f64, pr: f64) -> (f64, f64, f64) {
    let r = pr * (1.0 - 0.299) * 2.0 + y;
    let b = pb * (1.0 - 0.114) * 2.0 + y;
    let g = (y - 0.299 * r - 0.114 * b) / 0.587;
    (r, g, b)
}
