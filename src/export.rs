use std::io::Write;
use std::process::{Command, Stdio};

use crate::buffer::Buffer;
use crate::render::{Rgba, Render, RenderOpt};

pub fn rgbas_to_u8s(block: &[Rgba], u8s: &mut [u8]) {
    for i in 0..block.len() {
        u8s[i * 4 + 2] = (block[i].0.min(1.0).max(0.0) * 255.99).floor() as u8;
        u8s[i * 4 + 1] = (block[i].1.min(1.0).max(0.0) * 255.99).floor() as u8;
        u8s[i * 4 + 0] = (block[i].2.min(1.0).max(0.0) * 255.99).floor() as u8;
        u8s[i * 4 + 3] = (block[i].3.min(1.0).max(0.0) * 255.99).floor() as u8;
    }
}

pub fn render_to_mp4(
    sec: f64,
    width: usize,
    height: usize,
    framerate: usize,
    buffer_frame_num: usize,
    render: &Render<Rgba>) {
    let frames: usize = (framerate as f64 * sec).floor() as usize;
    let mut render_buffer = vec![Rgba::default(); width * height * buffer_frame_num];
    let mut buffer = vec![0u8; width * height * 4];
    let mut child = Command::new("/bin/sh")
        .args(&[
            "-c",
            format!(
                "ffmpeg -f rawvideo -pix_fmt bgra -s {width}x{height} -r {framerate} -i - -pix_fmt yuv420p -y {output}",
                width = width,
                height = height,
                framerate = framerate,
                output = "out.mp4").as_str()])
        .stdin(Stdio::piped())
        .spawn()
        .expect("failed to execute child");
    let child_stdin = child.stdin.as_mut().expect("failed to get stdin");
    for f in 0..frames / buffer_frame_num {
        render.render(&RenderOpt {
            u_range: 0.0..1.0,
            u_res: width,
            v_range: 0.0..1.0,
            v_res: height,
            frame_range: (f * buffer_frame_num) as i32..((f + 1) * buffer_frame_num) as i32,
            framerate: framerate
        }, render_buffer.as_mut_slice());
        for i in 0..buffer_frame_num {
            let p = i * width * height;
            rgbas_to_u8s(&render_buffer[p..p + width * height], buffer.as_mut_slice());
            child_stdin.write_all(buffer.as_slice()).unwrap();
        }
    }
    {
        let start = (frames / buffer_frame_num) * buffer_frame_num;
        render.render(&RenderOpt {
            u_range: 0.0..1.0,
            u_res: width,
            v_range: 0.0..1.0,
            v_res: height,
            frame_range: start as i32..frames as i32,
            framerate: framerate
        }, render_buffer.as_mut_slice());
        for i in 0..frames - start {
            let p = i * width * height;
            rgbas_to_u8s(&render_buffer[p..p + width * height], buffer.as_mut_slice());
            child_stdin.write_all(buffer.as_slice()).unwrap();
        }
    }
    child.wait().expect("child process wasn't running");
}

pub fn render_to_buffer(ro: &RenderOpt, render: &Render<Rgba>) -> Buffer<Rgba> {
    println!("render start: {:#?}", ro);
    let start = std::time::Instant::now();

    let frame_num = (ro.frame_range.end - ro.frame_range.start) as usize;
    let mut vec = vec![Rgba::default(); ro.u_res * ro.v_res * frame_num];
    render.render(ro, vec.as_mut_slice());

    let duration = start.elapsed();
    println!("render end, took: {}.{:04} sec",
             duration.as_secs(), duration.subsec_nanos() / 1_000_000);

    Buffer {
        width: ro.u_res,
        height: ro.v_res,
        frame_num: frame_num,
        framerate: ro.framerate,
        vec: vec
    }
}
