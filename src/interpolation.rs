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