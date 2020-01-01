use crate::v::V;

pub trait Timed<T> {
    fn get_value(&self, time: f64) -> T;
}

impl<T: V> Timed<T> for T {
    fn get_value(&self, _time: f64) -> T {
        self.clone()
    }
}
