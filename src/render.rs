use crate::pixel::Rgba;
use crate::util::hsl_to_rgb;

#[derive(Debug, Copy, Clone)]
pub struct Range(pub f64, pub f64);

impl Range {
    pub fn unit() -> Range {
        Range(0.0, 1.0)
    }

    pub fn size(&self) -> f64 {
        self.1 - self.0
    }

    pub fn at(&self, rate: f64) -> f64 {
        (self.1 - self.0) * rate + self.0
    }
}

pub type Res = (usize, usize);

#[derive(Debug)]
pub struct RenderOpt {
    pub u_range: Range,
    pub u_res: usize,
    pub v_range: Range,
    pub v_res: usize,
    pub frame_range: std::ops::Range<i32>,
    pub framerate: usize,
}

pub trait Render<T> {
    fn sample(&self, u: f64, v: f64, time: f64, res: Res) -> T;

    fn render(&self, ro: &RenderOpt, buffer: &mut [T]) {
        let RenderOpt {u_range, u_res, v_range, v_res, frame_range, framerate, ..} = ro;
        let res = ((*u_res as f64 / u_range.size()) as usize, (*v_res as f64 / v_range.size()) as usize);
        for f in frame_range.start..frame_range.end {
            let time = f as f64 / *framerate as f64;
            for y in 0..*v_res {
                let v = y as f64 / *v_res as f64;
                for x in 0..*u_res {
                    let u = x as f64 / *u_res as f64;
                    buffer[(f - frame_range.start) as usize * u_res * v_res + y * u_res + x] =
                        self.sample(u_range.at(u), v_range.at(v), time, res);
                }
            }
        }
    }
}

impl<T> Render<T> for Box<dyn Render<T>> {
    #[inline(always)]
    fn sample(&self, u: f64, v: f64, time: f64, res: Res) -> T {
        self.as_ref().sample(u, v, time, res)
    }

    #[inline(always)]
    fn render(&self, ro: &RenderOpt, buffer: &mut [T]) {
        self.as_ref().render(ro, buffer);
    }
}
impl<T> Render<T> for std::rc::Rc<dyn Render<T>> {
    #[inline(always)]
    fn sample(&self, u: f64, v: f64, time: f64, res: Res) -> T {
        self.as_ref().sample(u, v, time, res)
    }

    #[inline(always)]
    fn render(&self, ro: &RenderOpt, buffer: &mut [T]) {
        self.as_ref().render(ro, buffer);
    }
}

pub struct Dummy();

impl Render<Rgba> for Dummy {
    fn sample(&self, u: f64, v: f64, time: f64, _res: Res) -> Rgba {
        let (r, g, b) = hsl_to_rgb(time * 0.3, u, v);
        Rgba(r, g, b, 1.0)
    }

    fn render(&self, ro: &RenderOpt, buffer: &mut [Rgba]) {
        let RenderOpt {u_res, v_res, frame_range, framerate, ..} = ro;
        for f in frame_range.start..frame_range.end {
            for v in 0..*v_res {
                for u in 0..*u_res {
                    let (r, g, b) = hsl_to_rgb(f as f64 * 0.3 / *framerate as f64, u as f64 / *u_res as f64, v as f64 / *v_res as f64);
                    buffer[(f - frame_range.start) as usize * u_res * v_res + v * u_res + u] =
                        Rgba(r, g, b, 1.0);
                }
            }
        }
    }
}

impl Clone for RenderOpt {
    fn clone(&self) -> Self {
        RenderOpt {
            u_range: self.u_range.clone(),
            u_res: self.u_res,
            v_range: self.v_range.clone(),
            v_res: self.v_res,
            frame_range: self.frame_range.start..self.frame_range.end,
            framerate: self.framerate
        }
    }
}

// TODO: test
