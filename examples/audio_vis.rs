use kantera::{
    pixel::Rgba,
    export::render_to_mp4,
    renders::functional_render::FunctionalRender,
    audio_buffer::AudioBuffer,
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
        duration, 320, 240, 30, 1,
        "audio_vis_only_video.mp4",
        &(FunctionalRender(Box::new(move |ro, time, buffer| {
            let ab_pos = (time * sample_rate as f64) as usize;
            let mut input: Vec<Complex<f32>> =
                audio_buffer.vec[0]
                .iter().cycle().skip(ab_pos).take(fs)
                .map(|x| Complex::new(*x, 0.0)).collect();
            let mut output = vec![Complex::zero(); fs];
            fft.process(&mut input[0..fs], &mut output);

            let w = ro.res_x;
            let h = ro.res_y;
            for y in 0..h {
                let d = (y as f64 / h as f64) * 2.0 - 1.0;
                for x in 0..w {
                    let u = x as f64 / w as f64;
                    let i = ((10.0f64).powf(u) * fs as f64 / 20.0).floor() as usize;
                    let v1 = (output[i].norm() as f64).log10() * 0.15 + 0.2;
                    let v2 = (1.0 - (input[x].re as f64 - d).abs() * h as f64).max(0.0);
                    let v = v1.max(v2);
                    buffer[y * w + x] = Rgba(v, v, v, 1.0);
                }
            }
        }))));

    ffmpeg::combine("audio_vis_only_video.mp4", "scc.mp3", "audio_vis.mp4", true);

    println!("done!");
}
