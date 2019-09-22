fn main() {
    let audio_buffer = kantera::audio_buffer::make_audio(5.0);
    kantera::ffmpeg::export_audio(&audio_buffer, "a.mp3", true);
}
