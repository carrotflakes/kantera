use crate::lerp::Lerp;

pub trait Timed<T> {
    fn get_value(&self, time: f64) -> T;
}

impl<T: Lerp> Timed<T> for T {
    fn get_value(&self, _time: f64) -> T {
        self.clone()
    }
}
