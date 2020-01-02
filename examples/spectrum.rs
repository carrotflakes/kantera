use kantera::{
    pixel::Rgba,
    export::render_to_mp4,
    renders::sample::Sample,
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

    let mut planner: FFTplanner<f32> = FFTplanner::new(false);
    let fs = 4096;
    let fft = planner.plan_fft(fs);

    let mut os = vec![];

    for i in 0..600 {
        let mut input: Vec<Complex<f32>> = audio_buffer.vec[0]
            .iter().cycle().skip(44100 / 4 * i).take(fs).map(|x| Complex::new(*x, 0.0)).collect();
        let mut output = vec![Complex::zero(); fs];
        fft.process(&mut input[0..fs], &mut output);
        os.push(output.to_vec());
    }

    render_to_mp4(
        5.0, 320, 240, 30, 1,
        "spectrum.mp4",
        &(Box::new(move |u: f64, v: f64, time: f64, (_w, _h): (usize, usize)| {
            let i = (v * 240.0 + time * 50.0).floor() as usize;
            let j = ((10.0f64).powf(u) * fs as f64 / 20.0).floor() as usize;
            let v = (os[i][j].norm() as f64).log10() * 0.15 + 0.2;
            Rgba(v, v, v, 1.0)
        }) as Sample<Rgba>));

    println!("done!");
}
