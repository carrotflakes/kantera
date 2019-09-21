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

/*
pub fn import(file_path: &str) -> Buffer {
    let child = Command::new("/bin/sh")
            .args(&[
                "-c",
                format!(
                    "ffmpeg -i {input} -f image2pipe -pix_fmt bgra -vcodec rawvideo",
                    width = width,
                    height = height,
                    framerate = framerate,
                    output = file_name).as_str()])
            .stdin(Stdio::piped())
            .stdout(if debug { Stdio::inherit() } else { Stdio::null() })
            .stderr(if debug { Stdio::inherit() } else { Stdio::null() })
            .spawn()
            .expect("failed to execute child");
}*/

#[derive(Debug)]
pub struct VideoInfo {
    streams: Vec<StreamInfo>
}

#[derive(Debug)]
enum StreamInfo {
    Video {
        width: usize,
        height: usize,
    },
    Unknown {
        codec_type: String,
    }
}

use std::io::{BufRead, BufReader};

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
    let stdout_reader = BufReader::new(stdout);

    let mut streams = Vec::new();
    let mut codec_type: Option<String> = None;
    let mut width: Option<usize> = None;
    let mut height: Option<usize> = None;
    for line in stdout_reader.lines() {
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
        if line == "[/STREAM]" {
            if codec_type == Some("video".to_string()) {
                streams.push(StreamInfo::Video {
                    width: width.unwrap(),
                    height: height.unwrap()
                });
            } else {
                streams.push(StreamInfo::Unknown {
                    codec_type: codec_type.unwrap().to_string()
                });
            }
            codec_type = None;
            width = None;
            height = None;
        }
    }
    VideoInfo {
        streams
    }
}
