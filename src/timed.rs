use std::rc::Rc;
use crate::lerp::Lerp;

pub trait Timed<T> {
    fn get_value(&self, time: f64) -> T;
}

impl<T: Lerp> Timed<T> for T {
    #[inline(always)]
    fn get_value(&self, _time: f64) -> T {
        self.clone()
    }
}

impl<T: 'static> Timed<T> for Rc<dyn Timed<T>> {
    #[inline(always)]
    fn get_value(&self, time: f64) -> T {
        self.as_ref().get_value(time)
    }
}

#[derive(Debug)]
pub struct Cycle<T, U: Timed<T>> {
    timed: U,
    duration: f64,
    phantom: std::marker::PhantomData<T>
}

impl<T, U: Timed<T>> Cycle<T, U> {
    pub fn new(timed: U, duration: f64) -> Self {
        Cycle {
            timed,
            duration,
            phantom: std::marker::PhantomData
        }
    }
}

impl<T, U: Timed<T>> Timed<T> for Cycle<T, U> {
    fn get_value(&self, time: f64) -> T {
        self.timed.get_value(time % self.duration)
    }
}
