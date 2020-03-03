use crate::lerp::Lerp;

#[derive(Debug, Copy, Clone, PartialEq)]
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

impl std::ops::Add for Rgba {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Rgba(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2, self.3 + rhs.3)
    }
}

impl std::ops::Mul<f64> for Rgba {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self {
        Rgba(self.0 * rhs, self.1 * rhs, self.2 * rhs, self.3 * rhs)
    }
}

impl Lerp for Rgba {}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct RgbU8(pub u8, pub u8, pub u8);

impl Default for RgbU8 {
    fn default() -> Self {
        RgbU8(0, 0, 0)
    }
}

impl RgbU8 {
    pub fn normal_blend(&self, rhs: &RgbU8, alpha: f64) -> RgbU8 {
        RgbU8(
            (self.0 as f64 * (1.0 - alpha) + rhs.0 as f64 * alpha) as u8,
            (self.1 as f64 * (1.0 - alpha) + rhs.1 as f64 * alpha) as u8,
            (self.2 as f64 * (1.0 - alpha) + rhs.2 as f64 * alpha) as u8)
    }
}

impl From<RgbU8> for Rgba {
    fn from(p: RgbU8) -> Rgba {
        Rgba(
            p.0 as f64 / 255.0,
            p.1 as f64 / 255.0,
            p.2 as f64 / 255.0,
            1.0)
    }
}

// TODO: HSV
