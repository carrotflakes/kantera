use crate::pixel::Rgba;
use crate::util::hsl_to_rgb;

pub type Res = (usize, usize);

#[derive(Debug)]
pub struct RenderOpt {
    pub x_range: std::ops::Range<i32>,
    pub y_range: std::ops::Range<i32>,
    pub res_x: usize,
    pub res_y: usize,
    pub frame_range: std::ops::Range<i32>,
    pub framerate: usize,
}

pub trait Render<T> {
    fn sample(&self, u: f64, v: f64, time: f64, res: Res) -> T;

    fn render(&self, ro: &RenderOpt, buffer: &mut [T]) {
        let RenderOpt {x_range, y_range, res_x, res_y, frame_range, framerate, ..} = ro;
        let x_size = (x_range.end - x_range.start) as usize;
        let y_size = (y_range.end - y_range.start) as usize;
        let res = (*res_x, *res_y);
        for f in frame_range.start..frame_range.end {
            let time = f as f64 / *framerate as f64;
            for y in y_range.clone() {
                let v = y as f64 / *res_y as f64;
                for x in x_range.clone() {
                    let u = x as f64 / *res_x as f64;
                    buffer[(f - frame_range.start) as usize * x_size * y_size + (y - y_range.start) as usize * x_size + (x - x_range.start) as usize] =
                        self.sample(u, v, time, res);
                }
            }
        }
    }

    fn duration(&self) -> f64 {
        std::f64::INFINITY
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

    #[inline(always)]
    fn duration(&self) -> f64 {
        self.as_ref().duration()
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

    #[inline(always)]
    fn duration(&self) -> f64 {
        self.as_ref().duration()
    }
}

pub struct Dummy();

impl Render<Rgba> for Dummy {
    fn sample(&self, u: f64, v: f64, time: f64, _res: Res) -> Rgba {
        let (r, g, b) = hsl_to_rgb(time * 0.3, u, v);
        Rgba(r, g, b, 1.0)
    }

    fn render(&self, ro: &RenderOpt, buffer: &mut [Rgba]) {
        let RenderOpt {x_range, y_range, res_x, res_y, frame_range, framerate, ..} = ro;
        let x_size = (x_range.end - x_range.start) as usize;
        let y_size = (y_range.end - y_range.start) as usize;
        for f in frame_range.clone() {
            for y in y_range.clone() {
                for x in x_range.clone() {
                    let (r, g, b) = hsl_to_rgb(f as f64 * 0.3 / *framerate as f64, x as f64 / *res_x as f64, y as f64 / *res_y as f64);
                    buffer[(f - frame_range.start) as usize * x_size * y_size + (y - y_range.start) as usize * x_size + (x - x_range.start) as usize] =
                        Rgba(r, g, b, 1.0);
                }
            }
        }
    }

    fn duration(&self) -> f64 {
        std::f64::INFINITY
    }
}

impl Clone for RenderOpt {
    fn clone(&self) -> Self {
        RenderOpt {
            x_range: self.x_range.clone(),
            y_range: self.y_range.clone(),
            res_x: self.res_x,
            res_y: self.res_y,
            frame_range: self.frame_range.clone(),
            framerate: self.framerate
        }
    }
}

// TODO: test
