use crate::buffer::Buffer;
use crate::pixel::Rgba;
use crate::render::{Range, Render, RenderOpt};

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
            u_range: Range::unit(),
            u_res: width,
            v_range: Range::unit(),
            v_res: height,
            frame_range: (f * buffer_frame_num) as i32..((f + 1) * buffer_frame_num) as i32,
            framerate: framerate
        }, buffer.as_mut_slice());
        exporter.push(&buffer);
    }
    {
        let start = (frames / buffer_frame_num) * buffer_frame_num;
        render.render(&RenderOpt {
            u_range: Range::unit(),
            u_res: width,
            v_range: Range::unit(),
            v_res: height,
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

    let frame_num = (ro.frame_range.end - ro.frame_range.start) as usize;
    let mut vec = vec![T::default(); ro.u_res * ro.v_res * frame_num];
    render.render(ro, vec.as_mut_slice());

    let duration = start.elapsed();
    if unsafe {DEBUG_PRINT} {
        println!("render end, took: {}.{:04} sec",
                duration.as_secs(), duration.subsec_nanos() / 1_000_000);
    }

    Buffer {
        width: ro.u_res,
        height: ro.v_res,
        frame_num: frame_num,
        framerate: ro.framerate,
        vec: vec
    }
}

use std::thread;

pub fn render_to_buffer_parallel<T: Default + Clone + Send, U: From<T>>(ro: &RenderOpt, render: &'static (dyn Render<T> + Send + Sync)) -> Buffer<U> {
    let frame_num = (ro.frame_range.end - ro.frame_range.start) as usize;
    let n = 4; // TODO
    let handles: Vec<_> = (0..n).map(|i| {
        let u_range = Range(i as f64 / n as f64 * ro.u_range.size() + ro.u_range.0, (i + 1) as f64 / n as f64 * ro.u_range.size() + ro.u_range.0);
        let ro = RenderOpt {
            u_range: u_range,
            u_res: (i + 1) * ro.u_res / n - i * ro.u_res / n,
            ..ro.clone()
        };
        thread::spawn(move || {
            let mut vec = vec![T::default(); ro.u_res * ro.v_res * frame_num];
            render.render(&ro, vec.as_mut_slice());
            vec
        })
    }).collect::<Vec<_>>();
    let vecs = handles.into_iter().map(|handle| handle.join().unwrap()).collect::<Vec<Vec<T>>>();

    let mut vec = Vec::with_capacity(ro.u_res * ro.v_res * frame_num);
    for f in 0..frame_num {
        for y in 0..ro.v_res {
            for x in 0..ro.u_res {
                let i = x * n / ro.u_res;
                let u_res = (i + 1) * ro.u_res / n - i * ro.u_res / n;
                vec.push(vecs[i][f * u_res * ro.v_res + y * u_res + (x - i * ro.u_res / n)].clone().into());
            }
        }
    }

    Buffer {
        width: ro.u_res,
        height: ro.v_res,
        frame_num: frame_num,
        framerate: ro.framerate,
        vec: vec
    }
}
