extern crate cairo;

pub use cairo::*;
use crate::pixel::Rgba;
use crate::image::Image;
use crate::buffer::Buffer;

pub fn render_image(width: usize, height: usize, builder: &dyn Fn(Context)) -> Image<Rgba> {
    let mut surface =
        ImageSurface::create(Format::ARgb32, width as i32, height as i32).unwrap();
    builder(Context::new(&surface).unwrap());
    (&mut surface).into()
}

pub fn render_buffer(
    width: usize,
    height: usize,
    framerate: usize,
    builder: &dyn Fn(WrapedContext)) -> Buffer<Rgba> {
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
        let width = surface.width() as usize;
        let height = surface.height() as usize;
        let size = width * height;
        let mut vec = Vec::with_capacity(size);
        let data = surface.data().unwrap();
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
    context: Option<Context>,
    surface: ImageSurface,
    images: &'a mut Vec<Image<Rgba>>
}

impl<'a> WrapedContext<'a> {
    pub fn new(surface: ImageSurface, images: &'a mut Vec<Image<Rgba>>) -> Self {
        WrapedContext {
            context: Some(Context::new(&surface).unwrap()),
            surface,
            images
        }
    }
    pub fn push(&mut self) {
        self.context = None;
        self.images.push((&mut self.surface).into());
        self.context = Some(Context::new(&self.surface).unwrap());
    }
}

impl<'a> core::ops::Deref for WrapedContext<'a> {
    type Target = Context;

    fn deref(&self) -> &Context {
        self.context.as_ref().unwrap()
    }
}
