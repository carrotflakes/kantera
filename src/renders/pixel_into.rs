use crate::render::{Res, Render, RenderOpt};
use std::marker::PhantomData;

pub struct PixelInto<T: Default + Clone, U: From<T>, R: Render<T>> {
    pub render: R,
    pub t: PhantomData<T>,
    pub u: PhantomData<U>
}

impl <T: Default + Clone, U: From<T>, R: Render<T>> Render<U> for PixelInto<T, U, R> {
    fn sample(&self, u: f64, v: f64, time: f64, res: Res) -> U {
        self.render.sample(u, v, time, res).into()
    }

    fn render(&self, ro: &RenderOpt, buffer: &mut [U]) {
        let buffer_len = buffer.len();
        let mut inner_buffer = vec![T::default(); buffer_len];
        self.render.render(ro, &mut inner_buffer);
        for i in 0..buffer_len {
            buffer[i] = inner_buffer[i].clone().into();
        }
    }
}

impl <T: Default + Clone, U: From<T>, R: Render<T>> PixelInto<T, U, R> {
    pub fn new(r: R) -> PixelInto<T, U, R> {
        PixelInto {
            render: r,
            t: PhantomData,
            u: PhantomData
        }
    }
}
