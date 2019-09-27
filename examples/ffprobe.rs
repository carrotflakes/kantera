fn main() {
    let s = kantera::ffmpeg::probe("demo_with_audio.mp4");
    println!("{:?}", s);
}
