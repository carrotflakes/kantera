extern crate cairo;

use cairo::{Context, ImageSurface, Format};
use crate::pixel::Rgba;
use crate::image::Image;

pub fn render_image(width: usize, height: usize, builder: &Fn(Context)) -> Image<Rgba> {
    let mut surface =
        ImageSurface::create(Format::ARgb32, width as i32, height as i32).unwrap();
    builder(Context::new(&surface));
    surface.into()
}

impl From<ImageSurface> for Image<Rgba> {
    fn from(mut surface: ImageSurface) -> Image<Rgba> {
        let width = surface.get_width() as usize;
        let height = surface.get_height() as usize;
        let size = width * height;
        let mut vec = Vec::with_capacity(size);
        let data = surface.get_data().unwrap();
        for i in 0..size {
            vec.push(Rgba(
                data[i * 4 + 2] as f64 / 255.0,
                data[i * 4 + 1] as f64 / 255.0,
                data[i * 4 + 0] as f64 / 255.0,
                data[i * 4 + 3] as f64 / 255.0
            ));
        }
        Image {
            width: width,
            height: height,
            vec: vec
        }
    }
}
