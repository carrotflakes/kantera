fn main() {
    let buffer = kantera::ffmpeg::import("demo.mp4");
    println!("{}x{}x{} len: {}!", buffer.width, buffer.height, buffer.frame_num, buffer.vec.len());
    kantera::export::render_to_mp4(
        5.0, buffer.width, buffer.height, buffer.framerate, 1, "copied_demo.mp4",
        &kantera::renders::playback::Playback {
            buffer: Box::new(buffer)
        });
    println!("done");
}
