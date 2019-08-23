#[derive(Copy, Clone)]
pub struct Rgba(pub f64, pub f64, pub f64, pub f64);

impl Default for Rgba {
    fn default() -> Self {
        Rgba(0.0, 0.0, 0.0, 1.0)
    }
}

impl Rgba {
    pub fn normal_blend(&self, rhs: &Rgba, alpha: f64) -> Rgba {
        let alpha = rhs.3 * alpha;
        Rgba(
            self.0 * (1.0 - alpha) + rhs.0 * alpha,
            self.1 * (1.0 - alpha) + rhs.1 * alpha,
            self.2 * (1.0 - alpha) + rhs.2 * alpha,
            1.0 - (1.0 - self.3) * (1.0 - alpha))
    }
}

pub struct RenderOpt {
    pub u_range: std::ops::Range<f64>,
    pub u_res: usize,
    pub v_range: std::ops::Range<f64>,
    pub v_res: usize,
    pub frame_range: std::ops::Range<i32>,
    pub framerate: usize,
}

pub trait Render<T> {
    fn sample(&self, u: f64, v: f64, time: f64) -> T;

    fn render(&self, ro: RenderOpt, buffer: &mut [T]) {
        let RenderOpt {u_range, u_res, v_range, v_res, frame_range, framerate, ..} = ro;
        for f in frame_range.start..frame_range.end {
            let time = f as f64 / framerate as f64;
            for y in 0..v_res {
                let v = y as f64 / v_res as f64;
                for x in 0..u_res {
                    let u = x as f64 / u_res as f64;
                    buffer[(f - frame_range.start) as usize * u_res * v_res + y * u_res + x] =
                        self.sample(
                            u * (u_range.end - u_range.start) + u_range.start,
                            v * (v_range.end - v_range.start) + v_range.start,
                            time);
                }
            }
        }
    }
}

pub struct Dummy();

impl Render<Rgba> for Dummy {
    fn sample(&self, u: f64, v: f64, time: f64) -> Rgba {
        let (r, g, b) = hsl_to_rgb(time * 0.3, u, v);
        Rgba(r, g, b, 1.0)
    }

    fn render(&self, ro: RenderOpt, buffer: &mut [Rgba]) {
        let RenderOpt {u_res, v_res, frame_range, framerate, ..} = ro;
        for f in frame_range.start..frame_range.end {
            for v in 0..v_res {
                for u in 0..u_res {
                    let (r, g, b) = hsl_to_rgb(f as f64 * 0.3 / framerate as f64, u as f64 / u_res as f64, v as f64 / v_res as f64);
                    buffer[(f - frame_range.start) as usize * u_res * v_res + v * u_res + u] =
                        Rgba(r, g, b, 1.0);
                }
            }
        }
    }
}

fn hsl_to_rgb(h: f64, s: f64, l: f64) -> (f64, f64, f64) {
    let max = l + (s * (1.0 - (2.0 * l - 1.0).abs())) / 2.0;
    let min = l - (s * (1.0 - (2.0 * l - 1.0).abs())) / 2.0;
    let h = (h.fract() + 1.0).fract();
    match (h * 6.0).floor() as i32 % 6 {
        0 => (max, min + (max - min) * h * 6.0, min),
        1 => (min + (max - min) * (1.0 / 3.0 - h) * 6.0, max, min),
        2 => (min, max, min + (max - min) * (h - 1.0 / 3.0) * 6.0),
        3 => (min, min + (max - min) * (2.0 / 3.0 - h) * 6.0, max),
        4 => (min + (max - min) * (h - 2.0 / 3.0) * 6.0, min, max),
        5 => (max, min, min + (max - min) * (1.0 - h) * 6.0),
        _ => (min, min, min)
    }
}

//impl Copy for RenderOpt {
//}

impl Clone for RenderOpt {
    fn clone(&self) -> Self {
        RenderOpt {
            u_range: self.u_range.start..self.u_range.end,
            u_res: self.u_res,
            v_range: self.v_range.start..self.v_range.end,
            v_res: self.v_res,
            frame_range: self.frame_range.start..self.frame_range.end,
            framerate: self.framerate
        }
    }
}

// TODO: test
