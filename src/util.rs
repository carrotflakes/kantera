pub fn hsl_to_rgb(h: f64, s: f64, l: f64) -> (f64, f64, f64) {
    let max = l + (s * (1.0 - (2.0 * l - 1.0).abs())) / 2.0;
    let min = l - (s * (1.0 - (2.0 * l - 1.0).abs())) / 2.0;
    let h = (h.fract() + 1.0).fract();
    match (h * 6.0).floor() as i32 % 6 {
        0 => (max, min + (max - min) * h * 6.0, min),
        1 => (min + (max - min) * (1.0 / 3.0 - h) * 6.0, max, min),
        2 => (min, max, min + (max - min) * (h - 1.0 / 3.0) * 6.0),
        3 => (min, min + (max - min) * (2.0 / 3.0 - h) * 6.0, max),
        4 => (min + (max - min) * (h - 2.0 / 3.0) * 6.0, min, max),
        5 => (max, min, min + (max - min) * (1.0 - h) * 6.0),
        _ => (min, min, min)
    }
}
