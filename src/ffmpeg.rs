use std::io::Write;
use std::process::{Command, Child, Stdio, ChildStdin};
use crate::pixel::Rgba;

pub struct Exporter {
    size: (usize, usize),
    child: Child,
    buffer: Vec<u8>
}

impl Exporter {
    pub fn new(width: usize, height: usize, framerate: usize, file_name: &str) -> Self {
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
