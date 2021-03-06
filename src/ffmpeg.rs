use std::io::Write;
use std::process::{Command, Child, Stdio};
use crate::pixel::{Rgba, RgbU8};

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
                    "ffmpeg -hide_banner -f rawvideo -pix_fmt bgra -s {width}x{height} -r {framerate} -i - -pix_fmt yuv420p -y {output}",
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

pub fn import(file_path: &str) -> Buffer<RgbU8> {
    let vi = probe(file_path);
    let (width, height, frame_num, framerate) = vi.get_video_info().unwrap();
    let mut child = Command::new("/bin/sh")
        .args(&[
            "-c",
            format!(
                "ffmpeg -hide_banner -i {input} -f image2pipe -pix_fmt rgb24 -vcodec rawvideo -",
                input = file_path).as_str()])
        .stdout(Stdio::piped())
        .spawn()
        .expect("failed to execute child");
    let stdout = child.stdout.as_mut().unwrap();
    let mut reader = BufReader::new(stdout);
    let mut vec = Vec::with_capacity(width * height * frame_num);
    let mut buf = vec![0u8; width * height * 3];
    for _ in 0..frame_num {
        if let Ok(_) = reader.read_exact(buf.as_mut()) {
            for i in 0..width * height {
                vec.push(RgbU8(buf[i * 3 + 0], buf[i * 3 + 1], buf[i * 3 + 2]));
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
            if let StreamInfo::Video {..} = stream {
                return Some(stream);
            }
        }
        None
    }

    pub fn get_audio_stream<'a>(&'a self) -> Option<&'a StreamInfo> {
        for stream in self.streams.iter() {
            if let StreamInfo::Audio {..} = stream {
                return Some(stream);
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
    Audio {
        channel_num: usize,
        sample_rate: usize,
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
    let mut channel_num: Option<usize> = None;
    let mut sample_rate: Option<usize> = None;
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
            framerate = Regex::new(r"(\d+)/1").unwrap().captures(r_frame_rate).map(
                |caps| caps.get(1).unwrap().as_str().parse().unwrap());
        }
        if line.starts_with("nb_frames=") {
            // NOTE: N/A is considered 1.
            frame_num = Some(line[10..].parse().unwrap_or(1));
        }
        if line.starts_with("channels=") {
            channel_num = Some(line[9..].parse().unwrap());
        }
        if line.starts_with("sample_rate=") {
            sample_rate = Some(line[12..].parse().unwrap());
        }
        if line == "[/STREAM]" {
            if codec_type == Some("video".to_string()) {
                streams.push(StreamInfo::Video {
                    width: width.unwrap(),
                    height: height.unwrap(),
                    frame_num: frame_num.unwrap(),
                    framerate: framerate.unwrap(),
                });
            } else if codec_type == Some("audio".to_string()) {
                streams.push(StreamInfo::Audio {
                    channel_num: channel_num.unwrap(),
                    sample_rate: sample_rate.unwrap(),
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
            channel_num = None;
            sample_rate = None;
        }
    }
    VideoInfo {
        streams
    }
}

use crate::audio_buffer::AudioBuffer;

pub fn export_audio(audio_buffer: &AudioBuffer<u16>, file_name: &str, debug: bool) {
    let mut child = Command::new("/bin/sh")
        .args(&[
            "-c",
            format!(
                "ffmpeg -hide_banner -f u16le -ar {sample_rate} -ac {channel_num} -i - {output} -y",
                sample_rate = audio_buffer.sample_rate,
                channel_num = audio_buffer.channel_num,
                output = file_name).as_str()])
        .stdin(Stdio::piped())
        .stdout(if debug { Stdio::inherit() } else { Stdio::null() })
        .stderr(if debug { Stdio::inherit() } else { Stdio::null() })
        .spawn()
        .expect("failed to execute child");

    let stdin = child.stdin.as_mut().expect("failed to get stdin");

    for i in 0..audio_buffer.sample_num {
        for j in 0..audio_buffer.channel_num {
            stdin.write_all(&audio_buffer.vec[j][i].to_le_bytes()).unwrap();
        }
    }
    child.wait().expect("child process wasn't running");
}

pub fn import_audio(file_path: &str) -> AudioBuffer<u16> {
    let vi = probe(file_path);
    let (channel_num, sample_rate) =
        if let Some(StreamInfo::Audio {channel_num, sample_rate}) = vi.get_audio_stream() {
            (*channel_num, *sample_rate)
        } else {
            panic!()
        };
    let mut child = Command::new("/bin/sh")
        .args(&[
            "-c",
            format!(
                "ffmpeg -hide_banner -i {input} -f u16le -ar {sample_rate} -ac {channel_num} -vcodec rawaudio -",
                input = file_path,
                sample_rate = sample_rate,
                channel_num = channel_num).as_str()])
        .stdout(Stdio::piped())
        .spawn()
        .expect("failed to execute child");
    let stdout = child.stdout.as_mut().unwrap();
    let mut vec: Vec<Vec<u16>> = (0..channel_num).map(|_| Vec::new()).collect();
    let mut i = 0;
    let mut buf = [0u8; 2];
    while let Ok(_) = stdout.read_exact(&mut buf) {
        vec[i % channel_num].push(u16::from_le_bytes(buf));
        i += 1;
    }
    for v in vec.iter_mut() {
        v.shrink_to_fit();
    }
    AudioBuffer {
        channel_num,
        sample_rate,
        sample_num: vec[0].len(),
        vec
    }
}

use crate::image::Image;

pub fn import_image(file_path: &str) -> Image<Rgba> {
    let vi = probe(file_path);
    let (width, height, _, _) = vi.get_video_info().unwrap();
    let mut child = Command::new("/bin/sh")
        .args(&[
            "-c",
            format!(
                "ffmpeg -hide_banner -i {input} -f image2pipe -pix_fmt bgra -vcodec rawvideo -",
                input = file_path).as_str()])
        .stdout(Stdio::piped())
        .spawn()
        .expect("failed to execute child");
    let stdout = child.stdout.as_mut().unwrap();
    let mut reader = BufReader::new(stdout);
    let mut vec = Vec::with_capacity(width * height);
    let mut buf = vec![0u8; width * height * 4];
    if let Ok(_) = reader.read_exact(buf.as_mut()) {
        for i in 0..width * height {
            vec.push(Rgba(
                buf[i * 4 + 2] as f64 / 255.0,
                buf[i * 4 + 1] as f64 / 255.0,
                buf[i * 4 + 0] as f64 / 255.0,
                buf[i * 4 + 3] as f64 / 255.0));
        }
    }
    Image {
        width,
        height,
        vec
    }
}

pub fn combine(video_file_path: &str, audio_file_path: &str, file_path: &str, debug: bool) {
    let mut child = Command::new("/bin/sh")
        .args(&[
            "-c",
            format!(
                "ffmpeg -hide_banner -i {audio} -i {video} -c:v copy -c:a copy -y -strict -2 {output}",
                audio = audio_file_path,
                video = video_file_path,
                output = file_path).as_str()])
        .stdin(Stdio::null())
        .stdout(if debug { Stdio::inherit() } else { Stdio::null() })
        .stderr(if debug { Stdio::inherit() } else { Stdio::null() })
        .spawn()
        .expect("failed to execute child");
    child.wait().expect("child process wasn't running");
}
