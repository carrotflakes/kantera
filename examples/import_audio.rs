fn main() {
    let audio_buffer = kantera::ffmpeg::import_audio("demo_with_audio.mp4");
    kantera::ffmpeg::export_audio(&audio_buffer, "b.mp3", true);
}
