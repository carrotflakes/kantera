use crate::buffer::Buffer;
use crate::pixel::Rgba;
use crate::render::{Render, RenderOpt};

pub fn rgbas_to_u8s(block: &[Rgba], u8s: &mut [u8]) {
    for i in 0..block.len() {
        u8s[i * 4 + 2] = (block[i].0.min(1.0).max(0.0) * 255.99).floor() as u8;
        u8s[i * 4 + 1] = (block[i].1.min(1.0).max(0.0) * 255.99).floor() as u8;
        u8s[i * 4 + 0] = (block[i].2.min(1.0).max(0.0) * 255.99).floor() as u8;
        u8s[i * 4 + 3] = (block[i].3.min(1.0).max(0.0) * 255.99).floor() as u8;
    }
}

#[cfg(feature = "ffmpeg")]
pub fn render_to_mp4(
    sec: f64,
    width: usize,
    height: usize,
    framerate: usize,
    buffer_frame_num: usize,
    file_name: &str,
    render: &dyn Render<Rgba>) {
    let frames: usize = (framerate as f64 * sec).floor() as usize;
    let mut buffer = vec![Rgba::default(); width * height * buffer_frame_num];
    let mut exporter = crate::ffmpeg::Exporter::new(width, height, framerate, file_name, true);
    for f in 0..frames / buffer_frame_num {
        render.render(&RenderOpt {
            x_range: 0..width as i32,
            y_range: 0..height as i32,
            res_x: width,
            res_y: height,
            frame_range: (f * buffer_frame_num) as i32..((f + 1) * buffer_frame_num) as i32,
            framerate: framerate
        }, buffer.as_mut_slice());
        exporter.push(&buffer);
    }
    {
        let start = (frames / buffer_frame_num) * buffer_frame_num;
        render.render(&RenderOpt {
            x_range: 0..width as i32,
            y_range: 0..height as i32,
            res_x: width,
            res_y: height,
            frame_range: start as i32..frames as i32,
            framerate: framerate
        }, buffer.as_mut_slice());
        exporter.push(&buffer);
    }
    exporter.close();
}

pub static mut DEBUG_PRINT: bool = true;

pub fn render_to_buffer<T: Default + Clone>(ro: &RenderOpt, render: &dyn Render<T>) -> Buffer<T> {
    if unsafe {DEBUG_PRINT} {
        println!("render start: {:#?}", ro);
    }
    let start = std::time::Instant::now();

    let x_size = (ro.x_range.end - ro.x_range.start) as usize;
    let y_size = (ro.y_range.end - ro.y_range.start) as usize;
    let frame_num = (ro.frame_range.end - ro.frame_range.start) as usize;
    let mut vec = vec![T::default(); y_size * x_size * frame_num];
    render.render(ro, vec.as_mut_slice());

    let duration = start.elapsed();
    if unsafe {DEBUG_PRINT} {
        println!("render end, took: {}.{:04} sec",
                duration.as_secs(), duration.subsec_nanos() / 1_000_000);
    }

    Buffer {
        width: x_size,
        height: y_size,
        frame_num: frame_num,
        framerate: ro.framerate,
        vec: vec
    }
}

use std::thread;

pub fn render_to_buffer_parallel<T: Default + Clone + Send, U: From<T>>(ro: &RenderOpt, render: &'static (dyn Render<T> + Send + Sync)) -> Buffer<U> {
    let frame_num = (ro.frame_range.end - ro.frame_range.start) as usize;
    let x_size = (ro.x_range.end - ro.x_range.start) as usize;
    let y_size = (ro.y_range.end - ro.y_range.start) as usize;
    let n = 4i32; // TODO
    let handles: Vec<_> = (0..n).map(|i| {
        let x_range = (ro.x_range.end - ro.x_range.start) * i / n + ro.x_range.start..(ro.x_range.end - ro.x_range.start) * (i + 1) / n + ro.x_range.start;
        let ro = RenderOpt {
            x_range,
            ..ro.clone()
        };
        thread::spawn(move || {
            let mut vec = vec![T::default(); (ro.x_range.end - ro.x_range.start) as usize * y_size * frame_num];
            render.render(&ro, vec.as_mut_slice());
            vec
        })
    }).collect::<Vec<_>>();
    let vecs = handles.into_iter().map(|handle| handle.join().unwrap()).collect::<Vec<Vec<T>>>();

    let mut vec = Vec::with_capacity(y_size * x_size * frame_num);
    for f in 0..frame_num {
        for y in ro.y_range.clone() {
            for x in ro.x_range.clone() {
                let i = ((x - ro.x_range.start) * n) as usize / x_size;
                let x_size_ = (i + 1) * x_size / n as usize - i * x_size / n as usize;
                vec.push(vecs[i][f * x_size_ * y_size + (y - ro.y_range.start) as usize * x_size_ + ((x - ro.x_range.start) as usize - i * x_size / n as usize)].clone().into());
            }
        }
    }

    Buffer {
        width: x_size,
        height: y_size,
        frame_num: frame_num,
        framerate: ro.framerate,
        vec: vec
    }
}
