extern crate cairo;

use cairo::{Context, ImageSurface, Format};
use crate::pixel::Rgba;
use crate::image::Image;
use crate::buffer::Buffer;

pub fn render_image(width: usize, height: usize, builder: &Fn(Context)) -> Image<Rgba> {
    let mut surface =
        ImageSurface::create(Format::ARgb32, width as i32, height as i32).unwrap();
    builder(Context::new(&surface));
    (&mut surface).into()
}

pub fn render_buffer(
    width: usize,
    height: usize,
    framerate: usize,
    builder: &Fn(WrapedContext)) -> Buffer<Rgba> {
    let surface = ImageSurface::create(Format::ARgb32, width as i32, height as i32).unwrap();
    let mut images: Vec<Image<Rgba>> = vec![];

    builder(WrapedContext::new(surface, &mut images));

    let mut vec = vec![Rgba::default(); width * height * images.len()];
    for (i, image) in images.iter().enumerate() {
        for j in 0..width * height {
            vec[i * width * height + j] = image.vec[j];
        }
    }
    Buffer {
        width: width,
        height: height,
        frame_num: images.len(),
        framerate: framerate,
        vec: vec
    }
}

impl From<&mut ImageSurface> for Image<Rgba> {
    fn from(surface: &mut ImageSurface) -> Image<Rgba> {
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

pub struct WrapedContext<'a> {
    pub context: *mut Context,
    pub surface: ImageSurface,
    pub images: &'a mut Vec<Image<Rgba>>
}

impl<'a> WrapedContext<'a> {
    pub fn new(surface: ImageSurface, images: &'a mut Vec<Image<Rgba>>) -> Self {
        WrapedContext {
            context: Box::into_raw(Box::new(Context::new(&surface))),
            surface: surface,
            images: images
        }
    }
    pub fn push(&mut self) {
        unsafe { drop(Box::from_raw(self.context)) };
        self.images.push((&mut self.surface).into());
        self.context = Box::into_raw(Box::new(Context::new(&self.surface)));
    }
}

impl<'a> Drop for WrapedContext<'a> {
    fn drop(&mut self) {
        unsafe { drop(Box::from_raw(self.context)) };
    }
}

impl<'a> core::ops::Deref for WrapedContext<'a> {
    type Target = Context;

    fn deref(&self) -> &Context {
        unsafe { &*self.context }
    }
}
