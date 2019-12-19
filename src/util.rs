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

pub fn u32_noise(x: u32, y: u32, z: u32) -> u32 {
    // This is experimental implement.
    let w = x * 2777 + y * 2999 + z * 3252 + 0xa241ee91;
    (((x ^ w) + z) * ((y ^ w) + z) ^ (x + w) * (y + w) * (z + w)) + 0x9a6246f3
}

// https://mrl.nyu.edu/~perlin/noise/
pub fn noise(x: f64, y: f64, z: f64) -> f64 {
    let xx = x.floor() as usize & 255;
    let yy = y.floor() as usize & 255;
    let zz = z.floor() as usize & 255;
    let x = x - x.floor();
    let y = y - y.floor();
    let z = z - z.floor();
    let u = fade(x);
    let v = fade(y);
    let w = fade(z);
    let a = P[xx] + yy;
    let aa = P[a] + zz;
    let ab = P[a + 1] + zz;
    let b = P[xx + 1] + yy;
    let ba = P[b] + zz;
    let bb = P[b + 1] + zz;
    lerp(w,
         lerp(v,
              lerp(u, grad(P[aa], x, y, z), grad(P[ba], x - 1.0, y, z)),
              lerp(u, grad(P[ab], x, y - 1.0, z), grad(P[bb], x - 1.0, y - 1.0, z))),
         lerp(v,
              lerp(u, grad(P[aa + 1], x, y, z - 1.0), grad(P[ba + 1], x - 1.0, y, z - 1.0)),
              lerp(u, grad(P[ab + 1], x, y - 1.0, z - 1.0), grad(P[bb + 1], x - 1.0, y - 1.0, z - 1.0))))
}

fn fade(t: f64) -> f64 {
    t * t * t * (t * (t * 6.0 - 15.0) + 10.0)
}

fn lerp(t: f64, a: f64, b: f64) -> f64 {
    a + t * (b - a)
}

fn grad(hash: usize, x: f64, y: f64, z: f64) -> f64 {
    let h = hash & 15;
    let u = if h < 8 { x } else { y };
    let v = if h < 4 { y } else if h == 12 || h == 14 { x } else { z };
    (if (h & 1) == 0 { u } else { -u }) + (if (h & 2) == 0 { v } else { -v })
}

const PP: [usize; 256] = [
    151,160,137,91,90,15,
    131,13,201,95,96,53,194,233,7,225,140,36,103,30,69,142,8,99,37,240,21,10,23,
    190, 6,148,247,120,234,75,0,26,197,62,94,252,219,203,117,35,11,32,57,177,33,
    88,237,149,56,87,174,20,125,136,171,168, 68,175,74,165,71,134,139,48,27,166,
    77,146,158,231,83,111,229,122,60,211,133,230,220,105,92,41,55,46,245,40,244,
    102,143,54, 65,25,63,161, 1,216,80,73,209,76,132,187,208, 89,18,169,200,196,
    135,130,116,188,159,86,164,100,109,198,173,186, 3,64,52,217,226,250,124,123,
    5,202,38,147,118,126,255,82,85,212,207,206,59,227,47,16,58,17,182,189,28,42,
    223,183,170,213,119,248,152, 2,44,154,163, 70,221,153,101,155,167, 43,172,9,
    129,22,39,253, 19,98,108,110,79,113,224,232,178,185, 112,104,218,246,97,228,
    251,34,242,193,238,210,144,12,191,179,162,241, 81,51,145,235,249,14,239,107,
    49,192,214, 31,181,199,106,157,184, 84,204,176,115,121,50,45,127, 4,150,254,
    138,236,205,93,222,114,67,29,24,72,243,141,128,195,78,66,215,61,156,180];

lazy_static! {
    static ref P: Vec<usize> = PP.repeat(26);
}
