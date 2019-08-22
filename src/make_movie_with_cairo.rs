extern crate cairo;

use std::io::Write;
use std::process::{Command, Stdio};
use cairo::{ImageSurface, Format, Context};

fn make_make_frame(width: usize, height: usize) -> Box<FnMut(&mut [u8], usize)> {
    let mut surface = ImageSurface::create(Format::ARgb32, width as i32, height as i32).unwrap();

    Box::new(move |buffer: &mut [u8], time: usize| {
        {
            let ctx = Context::new(&surface);
            ctx.set_source_rgb(1.0, 1.0, 1.0);
            ctx.paint();

            ctx.move_to(width as f64 / 2.0, height as f64 / 2.0);
            ctx.line_to(width as f64 / 2.0 + (time as f64 / 30.0).cos() * 40.0,
                        height as f64 / 2.0 + (time as f64 / 30.0).sin() * 40.0);
            ctx.set_source_rgb(1.0, 0.0, 0.0);
            ctx.stroke();
        }
        let data = surface.get_data().unwrap();
        for i in 0..height*width*4 {
            buffer[i] = data[i];
        }
    })
}

fn main() {
    let width: usize = 320;
    let height: usize = 240;
    let framerate: usize = 30;
    let frames: usize = framerate * 5;
    let mut buffer = vec![0; width * height * 4];
    let mut child = Command::new("/bin/sh")
        .args(&[
            "-c",
            format!(
                "ffmpeg -f rawvideo -pix_fmt bgra -s {width}x{height} -i - -pix_fmt yuv420p -r {framerate} -y {output}",
                width = width,
                height = height,
                framerate = framerate,
                output = "out.mp4").as_str()])
        .stdin(Stdio::piped())
        .spawn()
        .expect("failed to execute child");
    let child_stdin = child.stdin.as_mut().expect("failed to get stdin");
    let mut fun = make_make_frame(width, height);
    for f in 0..frames {
        fun(buffer.as_mut_slice(), f);
        child_stdin.write_all(buffer.as_slice()).unwrap();
    }
    child.wait().expect("child process wasn't running");
}
