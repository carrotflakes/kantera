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

#[derive(Debug)]
pub struct Sine<T: Timed<f64>> {
    initial_phase: f64,
    frequency: f64,
    amplitude: T
}

impl<T: Timed<f64>> Sine<T> {
    pub fn new(initial_phase: f64, frequency: f64, amplitude: T) -> Self {
        Sine {
            initial_phase,
            frequency,
            amplitude
        }
    }
}

impl<T: Timed<f64>> Timed<f64> for Sine<T> {
    fn get_value(&self, time: f64) -> f64 {
        ((self.initial_phase + time) * self.frequency * std::f64::consts::PI * 2.0).sin() * self.amplitude.get_value(time)
    }
}
