#[derive(Copy, Clone)]
pub struct Rgba(pub f64, pub f64, pub f64, pub f64);

impl Default for Rgba {
    fn default() -> Self {
        Rgba(0.0, 0.0, 0.0, 1.0)
    }
}

impl Rgba {
    pub fn normal_blend(&self, rhs: &Rgba, alpha: f64) -> Rgba {
        let alpha = rhs.3 * alpha;
        Rgba(
            self.0 * (1.0 - alpha) + rhs.0 * alpha,
            self.1 * (1.0 - alpha) + rhs.1 * alpha,
            self.2 * (1.0 - alpha) + rhs.2 * alpha,
            1.0 - (1.0 - self.3) * (1.0 - alpha))
    }
}
