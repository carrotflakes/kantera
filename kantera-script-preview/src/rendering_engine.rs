use kantera::{
    audio_render::AudioRender,
    buffer::Buffer,
    export::render_to_buffer_parallel,
    pixel::Rgba,
    render::{Render, RenderOpt},
    script::{r, Runtime, Val, ValInterface},
};
use notify::{watcher, DebouncedEvent, RecursiveMode, Watcher};
use std::cell::RefCell;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::rc::Rc;
use std::sync::{mpsc::channel, Arc, Mutex};
use std::thread;
use std::time::Duration;

pub type RRenderingEngine = Arc<Mutex<RenderingEngine>>;

const FRAMERATE_DEFAULT: usize = 30;
const SAMPLERATE_DEFAULT: usize = 16000;

pub struct RenderingEngine {
    directory_path: PathBuf,
    main_path: PathBuf,
    current_frame: Option<i32>,
    render: Option<Rc<dyn Render<Rgba>>>,
    audio_render: Option<Rc<dyn AudioRender>>,
    framerate: usize,
    samplerate: usize,
    start_frame: i32,
    end_frame: Option<i32>,
    loop_: bool,
    size: (usize, usize),
    rt_cache: Val,
    frame_bin: Option<Vec<u8>>,
    audio_frame_bin: Option<Vec<u8>>,
}

impl RenderingEngine {
    pub fn new(directory_path: PathBuf) -> RenderingEngine {
        let main_path = {
            let mut p = directory_path.clone();
            p.push("main.ks");
            p
        };

        RenderingEngine {
            directory_path,
            main_path,
            current_frame: Some(0),
            render: None,
            audio_render: None,
            framerate: 30,
            samplerate: 44100,
            start_frame: 0,
            end_frame: None,
            loop_: false,
            size: (300, 400),
            rt_cache: r(RefCell::new(HashMap::<String, Val>::new())),
            frame_bin: None,
            audio_frame_bin: None,
        }
    }

    pub fn get_samplerate(&self) -> usize {
        self.samplerate
    }

    pub fn get_current_frame(&self) -> i32 {
        self.current_frame.unwrap_or_default()
    }

    pub fn get_frame_bin<'a>(&'a self) -> Option<&'a Vec<u8>> {
        self.frame_bin.as_ref()
    }

    pub fn get_audio_frame_bin<'a>(&'a self) -> Option<&'a Vec<u8>> {
        self.audio_frame_bin.as_ref()
    }

    fn render_loop(re: Arc<Mutex<RenderingEngine>>) {
        loop {
            let framerate = {
                let mut re = re.lock().unwrap();
                re.render_frame();
                re.framerate as u64
            };
            thread::sleep(Duration::from_millis(1000 / framerate));
        }
    }

    fn watch(re: Arc<Mutex<RenderingEngine>>) {
        let directory_path = re.lock().unwrap().directory_path.clone();

        let (tx, rx) = channel();
        let mut watcher = watcher(tx, Duration::from_secs(1)).unwrap();
        watcher
            .watch(directory_path, RecursiveMode::Recursive)
            .unwrap();

        loop {
            match rx.recv() {
                Ok(event) => {
                    println!("{:?}", event);
                    if let DebouncedEvent::Write(_path) = event {
                        re.lock().unwrap().run();
                    }
                }
                Err(e) => println!("watch error: {:?}", e),
            }
        }
    }

    fn run(&mut self) {
        match fs::read_to_string(&self.main_path) {
            Ok(src) => {
                self.run_script(&src);
            }
            Err(_) => {}
        }
    }

    pub fn start(re: Arc<Mutex<RenderingEngine>>) {
        re.lock().unwrap().run();

        {
            let re = re.clone();
            thread::spawn(move || RenderingEngine::render_loop(re));
        }
        {
            let re = re.clone();
            thread::spawn(move || RenderingEngine::watch(re));
        }
    }

    fn render_frame(&mut self) {
        if let Some(frame) = self.current_frame {
            if let Some(ref render) = self.render {
                let (width, height) = self.size;
                let buffer = {
                    let render = unsafe {
                        std::mem::transmute::<
                            &dyn Render<Rgba>,
                            &'static (dyn Render<Rgba> + Send + Sync),
                        >(render)
                    };
                    render_to_buffer_parallel(
                        &RenderOpt {
                            x_range: 0..width as i32,
                            y_range: 0..height as i32,
                            res_x: width,
                            res_y: height,
                            frame_range: frame..frame + 1,
                            framerate: self.framerate,
                        },
                        render,
                    ) as Buffer<Rgba>
                };
                let mut buf: Vec<u8> = vec![0; buffer.vec.len() * 4];
                for i in 0..buffer.vec.len() {
                    buf[i * 4 + 0] = (buffer.vec[i].0.min(1.0).max(0.0) * 255.99).floor() as u8;
                    buf[i * 4 + 1] = (buffer.vec[i].1.min(1.0).max(0.0) * 255.99).floor() as u8;
                    buf[i * 4 + 2] = (buffer.vec[i].2.min(1.0).max(0.0) * 255.99).floor() as u8;
                    buf[i * 4 + 3] = (buffer.vec[i].3.min(1.0).max(0.0) * 255.99).floor() as u8;
                }
                let mut bin = Vec::new();
                image::png::PNGEncoder::new(&mut bin)
                    .encode(&buf, width as u32, height as u32, image::RGBA(8))
                    .unwrap();
                self.frame_bin = Some(bin);
            }

            if let Some(ref audio_render) = self.audio_render {
                let sample_rate = self.samplerate;
                let ro = kantera::audio_render::AudioRenderOpt {
                    sample_rate: sample_rate,
                    sample_range: frame as i64 * sample_rate as i64 / self.framerate as i64
                        ..(frame as i64 + 1) * sample_rate as i64 / self.framerate as i64,
                };
                let vec = audio_render.render(&ro);
                let mut bin = Vec::new();
                use std::io::Write;
                for v in vec.iter() {
                    let v =
                        ((v.min(1.0).max(-1.0) + 1.0) / 2.0 * std::u16::MAX as f64).round() as u16;
                    bin.write(&v.to_le_bytes()).unwrap();
                }
                self.audio_frame_bin = Some(bin);
            }

            //ctx.text(format!(r#"{{"type":"sync","frame":{}}}"#, frame));
            let frame = frame + 1;
            self.current_frame = Some(frame);
            if let Some(end_frame) = self.end_frame {
                if end_frame <= frame {
                    if self.loop_ {
                        self.current_frame = Some(self.start_frame);
                    } else {
                        self.current_frame = None;
                    }
                }
            }
        }
    }

    fn make_runtime(&mut self) -> Runtime {
        let mut rt = Runtime::new();
        rt.insert("framerate", r(FRAMERATE_DEFAULT as i32));
        rt.insert("samplerate", r(SAMPLERATE_DEFAULT as i32));
        rt.insert("frame_size", r(vec![r(600 as i32), r(400 as i32)]));
        rt.insert("start_frame", r(0i32));
        rt.insert("end_frame", r(false));
        rt.insert("loop", r(false));
        rt.insert("frame_height", r(400 as i32));
        rt.insert("__rt_cache", self.rt_cache.clone());
        rt
    }

    fn run_script(&mut self, src: &str) {
        let mut rt = self.make_runtime();
        match rt.re(&src) {
            Ok(_) => {
                self.frame_bin = None;
                self.audio_frame_bin = None;
                self.render = rt
                    .get("video")
                    .and_then(|val| val.ref_as::<Rc<dyn Render<Rgba>>>().cloned());
                self.audio_render = rt
                    .get("audio")
                    .and_then(|val| val.ref_as::<Rc<dyn AudioRender>>().cloned());
                if let Some(val) = rt.get("framerate") {
                    let framerate = *val.ref_as::<i32>().unwrap();
                    self.framerate = framerate.min(120).max(1) as usize;
                }
                if let Some(val) = rt.get("samplerate") {
                    let samplerate = *val.ref_as::<i32>().unwrap();
                    self.samplerate = samplerate.min(48000).max(4000) as usize;
                }
                if let Some(val) = rt.get("frame_size") {
                    let val = val;
                    let vec = val.ref_as::<Vec<Val>>().unwrap();
                    let width = *vec[0].ref_as::<i32>().unwrap();
                    let height = *vec[1].ref_as::<i32>().unwrap();
                    self.size = (width.max(0) as usize, height.max(0) as usize);
                }
                self.start_frame = rt
                    .get("start_frame")
                    .and_then(|val| val.ref_as::<i32>().copied())
                    .unwrap_or(0);
                self.end_frame = rt
                    .get("end_frame")
                    .and_then(|val| val.ref_as::<i32>().copied())
                    .map(|val| val.max(1));
                self.loop_ = rt
                    .get("loop")
                    .and_then(|val| val.ref_as::<bool>().copied())
                    .unwrap_or(false);
                self.current_frame = Some(self.start_frame);
                let channel_num = self
                    .audio_render
                    .as_ref()
                    .map(|r| r.channel_num())
                    .unwrap_or(0);
            }
            Err(mes) => println!("{}", mes),
        }
    }
}

unsafe impl Send for RenderingEngine {}
unsafe impl Sync for RenderingEngine {}
