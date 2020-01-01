pub trait Lerp: Copy {
    fn lerp(&self, other: &Self, v: f64) -> Self;
    fn bezier(left: &Self, left_handle: &Self, right: &Self, right_handle: &Self, v: f64) -> Self;
}

impl Lerp for f64  {
    #[inline(always)]
    fn lerp(&self, other: &Self, v: f64) -> Self {
        self * (1.0 - v) + other * v
    }
    fn bezier(left: &Self, left_handle: &Self, right: &Self, right_handle: &Self, v: f64) -> Self {
        let iv = 1.0 - v;
        (left + left_handle * v) * iv + (right + right_handle * iv) * v
    }
}
