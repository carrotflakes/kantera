use kantera::{
    pixel::Rgba,
    export::render_to_mp4,
    renders::functional_render::FunctionalRender,
    audio_buffer::AudioBuffer,
    util::hsl_to_rgb,
    ffmpeg,
};

use rustfft::{
    FFTplanner,
    num_complex::Complex,
    num_traits::Zero
};

fn main() {
    let audio_buffer: AudioBuffer<f32> = (&ffmpeg::import_audio("scc.mp3")).into();
    let sample_rate = audio_buffer.sample_rate;

    let fs = 4096;
    let mut planner: FFTplanner<f32> = FFTplanner::new(false);
    let fft = planner.plan_fft(fs);

    let duration = audio_buffer.sample_num as f64 / sample_rate as f64;
    println!("duration: {}", duration);
    render_to_mp4(
        duration, 320 * 2, 240 * 2, 30, 1,
        "audio_vis2_only_video.mp4",
        &(FunctionalRender(Box::new(move |ro, time, buffer| {
            let ab_pos = (time * sample_rate as f64) as usize;
            let mut input: Vec<Complex<f32>> =
                audio_buffer.vec[0]
                .iter().cycle().skip(ab_pos).take(fs)
                .map(|x| Complex::new(*x, 0.0)).collect();
            let mut output = vec![Complex::zero(); fs];
            fft.process(&mut input[0..fs], &mut output);

            let DPI = std::f64::consts::PI * 2.0;
            let res = 24.0;
            let n = 9;
            let w = ro.u_res;
            let h = ro.v_res;
            for y in 0..h {
                let cv = (y as f64 / h as f64) * 2.0 - 1.0;
                for x in 0..w {
                    let cu = -(x as f64 - w as f64 / 2.0) as f64 / (h / 2) as f64;
                    let theta = cv.atan2(cu);
                    let r = cv.hypot(cu);
                    let a = r * (n + 1) as f64 * 2.0 + (theta / DPI).fract();
                    let b = (a.floor() - (theta / DPI).fract()) / n as f64;
                    let c = 2.0f64.powf(b * n as f64 - 2.0) * 1.049 / res;
                    let d = if 0.001 < c && c < 1.0 {
                        ((output[((c * fs as f64 * 0.5).round() as i32) as usize].norm() as f64)
                         .log10() * 0.15 + 0.2).max(0.0)
                    } else {
                        0.0
                    };
                    buffer[y * w + x] = Rgba(d, d, d, 1.0);
                    /*
                    let i = (((10.0f64).powf(theta / DPI) * fs as f64 / 20.0).floor() as usize).min(fs / 2);
                    let pow = ((output[i].norm() as f64).log10() * 0.15 + 0.2).max(0.0);
                    let v1 = (pow * 50.0 + 1.0 - (r - 0.5).abs() * h as f64).max(0.0) / 30.0;
                    let v2 = 1.0 - (0.5 - (r * 4.0 + theta / DPI) % 1.0).abs() * 2.0;
                    let (r, g, b) = hsl_to_rgb(0.4 - v1, 0.5, v1.max(v2));
                    buffer[y * w + x] = Rgba(r, g, b, 1.0);*/
                }
            }
        }))));

    ffmpeg::combine("audio_vis2_only_video.mp4", "scc.mp3", "audio_vis2.mp4", true);

    println!("done!");
}
