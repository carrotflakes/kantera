use kantera::{
    ffmpeg,
    renders::{
        playback::Playback,
        pixel_into::PixelInto
    }
};

fn main() {
    let buffer = ffmpeg::import("demo.mp4");
    println!("{}x{}x{} len: {}!", buffer.width, buffer.height, buffer.frame_num, buffer.vec.len());
    kantera::export::render_to_mp4(
        5.0, buffer.width, buffer.height, buffer.framerate, 1, "copied_demo.mp4",
        &PixelInto::new(Playback {
            buffer: Box::new(buffer)
        }));
    println!("done");
}
