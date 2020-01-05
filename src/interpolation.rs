use crate::image::Image;
use crate::lerp::Lerp;

pub trait Interpolation<T: Lerp> {
    fn interpolate(&self, image: &Image<T>, x: f64, y: f64) -> T;
}

pub struct NearestNeighbor;

impl<T: Lerp> Interpolation<T> for NearestNeighbor {
    fn interpolate(&self, image: &Image<T>, x: f64, y: f64) -> T {
        if 0.0 <= x && x < image.width as f64 && 0.0 <= y && y < image.height as f64 {
            image.vec[y as usize * image.width + x as usize].clone()
        } else {
            panic!()
        }
    }
}

pub struct Bilinear;

impl<T: Lerp> Interpolation<T> for Bilinear {
    fn interpolate(&self, image: &Image<T>, x: f64, y: f64) -> T {
        if 0.0 <= x && x < image.width as f64 && 0.0 <= y && y < image.height as f64 {
            let fx = x.fract();
            let fy = y.fract();
            // TODO: dirty hack!
            let x_ceil = (x.ceil() as usize).min(image.width - 1);
            let y_ceil = (y.ceil() as usize).min(image.height - 1);
            image.vec[y.floor() as usize * image.width + x.floor() as usize].lerp(
                &image.vec[y.floor() as usize * image.width + x_ceil], fx)
                .lerp(
                    &image.vec[y_ceil * image.width + x.floor() as usize].lerp(
                        &image.vec[y_ceil * image.width + x_ceil], fx),
                    fy)
        } else {
            panic!()
        }
    }
}

pub struct Bicubic {
    p: f64,
    q: f64,
    r: f64,
    s: f64,
    t: f64,
    u: f64,
    v: f64,
    w: f64
}

impl Bicubic {
    pub fn new(b: f64, c: f64) -> Self {
        Bicubic {
            p: 2.0 - 1.5 * b - c,
            q: -3.0 + 2.0 * b + c,
            r: 0.0,
            s: 1.0 - (1.0 / 3.0) * b,
            t: -(1.0 / 6.0) * b - c,
            u: b + 5.0 * c,
            v: -2.0 * b - 8.0 * c,
            w: (4.0 / 3.0) * b + 4.0 * c
        }
    }
}

impl<T: Lerp> Interpolation<T> for Bicubic {
    fn interpolate(&self, image: &Image<T>, x: f64, y: f64) -> T {
        let cubic_bc = |x: f64| -> f64 {
            let ax = x.abs();
            if ax < 1.0 {
                ((self.p * ax + self.q) * ax + self.r) * ax + self.s
            } else if ax < 2.0 {
                ((self.t * ax + self.u) * ax + self.v) * ax + self.w
            } else {
                0.0
            }
        };
        let x_floor = x.floor() as i32;
        let y_floor = y.floor() as i32;
        let pixel = |dx: i32, dy: i32| -> T {
            let x = (x_floor + dx).max(0).min(image.width as i32 - 1);
            let y = (y_floor + dy).max(0).min(image.height as i32 - 1);
            image.vec[y as usize * image.width + x as usize]
        };
        let hx = [
            cubic_bc(x - (x_floor - 1) as f64),
            cubic_bc(x - x_floor as f64),
            cubic_bc((x_floor + 1) as f64 - x),
            cubic_bc((x_floor + 2) as f64 - x)
        ];
        let hy = [
            cubic_bc(y - (y_floor - 1) as f64),
            cubic_bc(y - y_floor as f64),
            cubic_bc((y_floor + 1) as f64 - y),
            cubic_bc((y_floor + 2) as f64 - y)
        ];
        (pixel(-1, -1).scale(hx[0]) + pixel(0, -1).scale(hx[1]) + pixel(1, -1).scale(hx[2]) + pixel(2, -1).scale(hx[3])).scale(hy[0])
        + (pixel(-1, 0).scale(hx[0]) + pixel(0, 0).scale(hx[1]) + pixel(1, 0).scale(hx[2]) + pixel(2, 0).scale(hx[3])).scale(hy[1])
        + (pixel(-1, 1).scale(hx[0]) + pixel(0, 1).scale(hx[1]) + pixel(1, 1).scale(hx[2]) + pixel(2, 1).scale(hx[3])).scale(hy[2])
        + (pixel(-1, 2).scale(hx[0]) + pixel(0, 2).scale(hx[1]) + pixel(1, 2).scale(hx[2]) + pixel(2, 2).scale(hx[3])).scale(hy[3])
    }
}
