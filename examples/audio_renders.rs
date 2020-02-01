use kantera::{
    audio_buffer::AudioBuffer,
    audio_render::{render_to_buffer},
    ffmpeg::export_audio
};

fn main() {
    let render = kantera::audio_render::Dummy(3.0);
    let audio_buffer = render_to_buffer(&render, 44100);
    let audio_buffer = AudioBuffer::<u16>::from(&audio_buffer);
    export_audio(&audio_buffer, "audio_renders.mp3", true);
}
