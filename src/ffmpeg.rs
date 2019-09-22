use std::io::Write;
use std::process::{Command, Child, Stdio, ChildStdin};
use crate::pixel::Rgba;

pub struct Exporter {
    size: (usize, usize),
    child: Child,
    buffer: Vec<u8>
}

impl Exporter {
    pub fn new(width: usize, height: usize, framerate: usize, file_name: &str, debug: bool) -> Self {
        let child = Command::new("/bin/sh")
            .args(&[
                "-c",
                format!(
                    "ffmpeg -f rawvideo -pix_fmt bgra -s {width}x{height} -r {framerate} -i - -pix_fmt yuv420p -y {output}",
                    width = width,
                    height = height,
                    framerate = framerate,
                    output = file_name).as_str()])
            .stdin(Stdio::piped())
            .stdout(if debug { Stdio::inherit() } else { Stdio::null() })
            .stderr(if debug { Stdio::inherit() } else { Stdio::null() })
            .spawn()
            .expect("failed to execute child");
        Exporter {
            size: (width, height),
            child: child,
            buffer: vec![0u8; width * height * 4]
        }
    }

    pub fn push(&mut self, vec: &[Rgba]) {
        let (width, height) = self.size;
        assert_eq!(vec.len() % (width * height), 0);
        let frame_num = vec.len() / (width * height);
        let child_stdin = self.child.stdin.as_mut().expect("failed to get stdin");

        for i in 0..frame_num {
            let p = i * width * height;
            rgbas_to_u8s(&vec[p..p + width * height], self.buffer.as_mut_slice());
            child_stdin.write_all(self.buffer.as_slice()).unwrap();
        }
    }

    pub fn close(&mut self) {
        self.child.wait().expect("child process wasn't running");
    }
}

pub fn rgbas_to_u8s(block: &[Rgba], u8s: &mut [u8]) {
    for i in 0..block.len() {
        u8s[i * 4 + 2] = (block[i].0.min(1.0).max(0.0) * 255.99).floor() as u8;
        u8s[i * 4 + 1] = (block[i].1.min(1.0).max(0.0) * 255.99).floor() as u8;
        u8s[i * 4 + 0] = (block[i].2.min(1.0).max(0.0) * 255.99).floor() as u8;
        u8s[i * 4 + 3] = (block[i].3.min(1.0).max(0.0) * 255.99).floor() as u8;
    }
}

pub fn u8s_to_rgbas(u8s: &[u8], block: &mut [Rgba]) {
    for i in 0..block.len() {
        block[i] = Rgba(
            u8s[i * 4 + 2] as f64 / 255.0,
            u8s[i * 4 + 1] as f64 / 255.0,
            u8s[i * 4 + 0] as f64 / 255.0,
            u8s[i * 4 + 3] as f64 / 255.0);
    }
}

use crate::buffer::Buffer;
use std::io::Read;

pub fn import(file_path: &str) -> Buffer<Rgba> {
    let vi = probe(file_path);
    let (width, height, frame_num, framerate) = vi.get_video_info().unwrap();
    let mut child = Command::new("/bin/sh")
        .args(&[
            "-c",
            format!(
                "ffmpeg -i {input} -f image2pipe -pix_fmt bgra -vcodec rawvideo -",
                input = file_path).as_str()])
        .stdout(Stdio::piped())
        .spawn()
        .expect("failed to execute child");
    let stdout = child.stdout.as_mut().unwrap();
    let mut reader = BufReader::new(stdout);
    let mut vec = Vec::with_capacity(width * height * frame_num);
    let mut buf = vec![0u8; width * height * 4];
    for _ in 0..frame_num {
        if let _ = reader.read_exact(buf.as_mut()) {
            for i in 0..width * height {
                vec.push(Rgba(
                    buf[i * 4 + 2] as f64 / 255.0,
                    buf[i * 4 + 1] as f64 / 255.0,
                    buf[i * 4 + 0] as f64 / 255.0,
                    buf[i * 4 + 3] as f64 / 255.0));
            }
        } else {
            break;
        }
    }
    vec.shrink_to_fit();
    Buffer {
        width: width,
        height: height,
        frame_num: frame_num,
        framerate: framerate,
        vec
    }
}

#[derive(Debug)]
pub struct VideoInfo {
    streams: Vec<StreamInfo>
}

impl VideoInfo {
    pub fn get_video_stream<'a>(&'a self) -> Option<&'a StreamInfo> {
        for stream in self.streams.iter() {
            match stream {
                StreamInfo::Video {..} => {
                    return Some(stream)
                },
                _ => ()
            }
        }
        None
    }

    pub fn get_video_info(&self) -> Option<(usize, usize, usize, usize)> {
        match self.get_video_stream() {
            Some(StreamInfo::Video {width, height, frame_num, framerate}) =>
                Some((*width, *height, *frame_num, *framerate)),
            _ => None
        }
    }
}

#[derive(Debug)]
pub enum StreamInfo {
    Video {
        width: usize,
        height: usize,
        frame_num: usize,
        framerate: usize,
    },
    Unknown {
        codec_type: String,
    }
}

use std::io::{BufRead, BufReader};
use regex::Regex;

pub fn probe(file_path: &str) -> VideoInfo {
    let mut child = Command::new("/bin/sh")
        .args(&[
            "-c",
            format!(
                "ffprobe {input} -hide_banner -show_streams",
                input = file_path).as_str()])
        .stderr(Stdio::null())
        .stdout(Stdio::piped())
        .spawn()
        .expect("failt to ffprobe");
    let stdout = child.stdout.as_mut().unwrap();
    let reader = BufReader::new(stdout);

    let mut streams = Vec::new();
    let mut codec_type: Option<String> = None;
    let mut width: Option<usize> = None;
    let mut height: Option<usize> = None;
    let mut frame_num: Option<usize> = None;
    let mut framerate: Option<usize> = None;
    for line in reader.lines() {
        let line = line.unwrap();
        if line.starts_with("codec_type=") {
            codec_type = Some(line[11..].to_string());
        }
        if line.starts_with("width=") {
            width = Some(line[6..].parse().unwrap());
        }
        if line.starts_with("height=") {
            height = Some(line[7..].parse().unwrap());
        }
        if line.starts_with("r_frame_rate=") {
            let r_frame_rate = &line[13..];
            let caps = Regex::new(r"(\d+)/1").unwrap().captures(r_frame_rate).unwrap();
            framerate = Some(caps.get(1).unwrap().as_str().parse().unwrap());
        }
        if line.starts_with("nb_frames=") {
            frame_num = Some(line[10..].parse().unwrap());
        }
        if line == "[/STREAM]" {
            if codec_type == Some("video".to_string()) {
                streams.push(StreamInfo::Video {
                    width: width.unwrap(),
                    height: height.unwrap(),
                    frame_num: frame_num.unwrap(),
                    framerate: framerate.unwrap(),
                });
            } else {
                streams.push(StreamInfo::Unknown {
                    codec_type: codec_type.unwrap().to_string()
                });
            }
            codec_type = None;
            width = None;
            height = None;
            frame_num = None;
            framerate = None;
        }
    }
    VideoInfo {
        streams
    }
}
