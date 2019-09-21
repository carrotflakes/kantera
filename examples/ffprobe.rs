fn main() {
    let s = kantera::ffmpeg::probe("demo.mp4");
    println!("{:?}", s);
}
