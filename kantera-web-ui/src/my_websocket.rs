use std::time::{Duration, Instant};

use actix::prelude::*;
use actix_web_actors::ws;

use kantera::{
    pixel::Rgba,
    buffer::Buffer,
    render::{Render, RenderOpt},
    audio_render::AudioRender,
    export::render_to_buffer_parallel,
    script::{Runtime, r, Val}
};
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;


const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);
const FRAMERATE_DEFAULT: usize = 30;
const SAMPLERATE_DEFAULT: usize = 16000;

pub struct MyWebSocket {
    hb: Instant,
    current_frame: Option<i32>,
    render: Option<Rc<dyn Render<Rgba>>>,
    audio_render: Option<Rc<dyn AudioRender>>,
    framerate: usize,
    samplerate: usize,
    start_frame: i32,
    end_frame: Option<i32>,
    loop_: bool,
    size: (usize, usize),
    render_at: Instant,
    rt_cache: Val
}

impl Actor for MyWebSocket {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.haertbeat(ctx);
        self.render_loop(ctx);
        ctx.text(r#"{"type":"log","log":"ready."}"#);
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for MyWebSocket {
    fn handle(
        &mut self,
        msg: Result<ws::Message, ws::ProtocolError>,
        ctx: &mut Self::Context,
    ) {
        match msg {
            Ok(ws::Message::Ping(msg)) => {
                self.hb = Instant::now();
                ctx.pong(&msg);
            }
            Ok(ws::Message::Pong(_)) => {
                self.hb = Instant::now();
            }
            Ok(ws::Message::Text(text)) => {
                println!("{:?}", text);
                if text.starts_with("script: ") {
                    let src = &text["script: ".len()..];
                    self.run_script(src, ctx);
                }
                if text.starts_with("render: ") {
                    let file_name = &text["script: ".len()..];
                    let file_path = format!("./tmp/{}", file_name);
                    if let Some(ref render) = self.render {
                        let duration = render.duration();
                        if duration.is_infinite() {
                            ctx.text(format!(r#"{{"type":"renderFailed","error":"Video duration must be finite"}}"#));
                            return;
                        }
                        kantera::export::render_to_mp4(
                            duration,
                            self.size.0,
                            self.size.1,
                            self.framerate,
                            10,
                            &file_path,
                            render.as_ref()
                        );
                        if let Some(ref audio_render) = self.audio_render {
                            if audio_render.duration().is_infinite() {
                                ctx.text(format!(r#"{{"type":"renderFailed","error":"Audio duration must be finite"}}"#));
                                return;
                            }
                            let buffer = kantera::audio_render::render_to_buffer(audio_render.as_ref(), self.samplerate);
                            kantera::ffmpeg::export_audio(&(&buffer).into(), "/tmp/kantera_audio.mp3", true);
                            if std::fs::rename(&file_path, "/tmp/kantera_video.mp4").is_err() {
                                ctx.text(format!(r#"{{"type":"renderFailed","error":"Internal error"}}"#));
                                return;
                            }
                            kantera::ffmpeg::combine("/tmp/kantera_video.mp4", "/tmp/kantera_audio.mp3", &file_path, true);
                            println!("Rendering done");
                        }
                        ctx.text(format!(r#"{{"type":"renderSucceeded","path":{:?}}}"#, format!("{}", file_path)));
                    } else {
                        ctx.text(format!(r#"{{"type":"renderFailed","error":"Render is None"}}"#));
                    }
                }
            },
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
            Ok(ws::Message::Close(_)) => ctx.stop(),
            _ => ctx.stop(),
        }
    }
}

impl MyWebSocket {
    pub fn new() -> Self {
        Self {
            hb: Instant::now(),
            current_frame: None,
            render: None,
            audio_render: None,
            framerate: FRAMERATE_DEFAULT,
            samplerate: SAMPLERATE_DEFAULT,
            start_frame: 0,
            end_frame: None,
            loop_: false,
            size: (600, 400),
            render_at: Instant::now(),
            rt_cache: r(RefCell::new(HashMap::<String, Val>::new()))
        }
    }

    fn haertbeat(&self, ctx: &mut <Self as Actor>::Context) {
        ctx.run_interval(HEARTBEAT_INTERVAL, |act, ctx| {
            if Instant::now().duration_since(act.hb) > CLIENT_TIMEOUT {
                println!("Websocket Client heartbeat failed, disconnecting!");
                ctx.stop();
                return;
            }
            ctx.ping(b"");
        });
    }

    fn render_loop(&mut self, ctx: &mut <Self as Actor>::Context) {
        if let Some(frame) = self.current_frame {
            if let Some(ref render) = self.render {
                let (width, height) = self.size;
                let buffer = {
                    let render = unsafe { std::mem::transmute::<&dyn Render<Rgba>, &'static (dyn Render<Rgba> + Send + Sync)>(render) };
                    render_to_buffer_parallel(&RenderOpt {
                        x_range: 0..width as i32,
                        y_range: 0..height as i32,
                        res_x: width,
                        res_y: height,
                        frame_range: frame..frame+1,
                        framerate: self.framerate
                    }, render) as Buffer<Rgba>
                };
                let mut buf: Vec<u8> = vec![0; buffer.vec.len() * 4];
                for i in 0..buffer.vec.len() {
                    buf[i * 4 + 0] = (buffer.vec[i].0.min(1.0).max(0.0) * 255.99).floor() as u8;
                    buf[i * 4 + 1] = (buffer.vec[i].1.min(1.0).max(0.0) * 255.99).floor() as u8;
                    buf[i * 4 + 2] = (buffer.vec[i].2.min(1.0).max(0.0) * 255.99).floor() as u8;
                    buf[i * 4 + 3] = (buffer.vec[i].3.min(1.0).max(0.0) * 255.99).floor() as u8;
                }
                let mut bin = Vec::new();
                image::png::PNGEncoder::new(&mut bin).encode(&buf, width as u32, height as u32, image::RGBA(8)).unwrap();
                ctx.text(r#"{"type":"frame"}"#);
                ctx.binary(bin);
            }

            if let Some(ref audio_render) = self.audio_render {
                let sample_rate = self.samplerate;
                let ro = kantera::audio_render::AudioRenderOpt {
                    sample_rate: sample_rate,
                    sample_range: frame as i64 * sample_rate as i64 / self.framerate as i64..(frame as i64 + 1) * sample_rate as i64 / self.framerate as i64
                };
                let vec = audio_render.render(&ro);
                let mut bin = Vec::new();
                use std::io::Write;
                for v in vec.iter() {
                    let v = ((v.min(1.0).max(-1.0) + 1.0) / 2.0 * std::u16::MAX as f64).round() as u16;
                    bin.write(&v.to_le_bytes()).unwrap();
                }
                ctx.text(r#"{"type":"audio"}"#);
                ctx.binary(bin);
            }

            ctx.text(format!(r#"{{"type":"sync","frame":{}}}"#, frame));
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

        // Schedule next render
        let desire_duration = Duration::from_millis(1000 / self.framerate as u64);
        self.render_at = (self.render_at + desire_duration).max(Instant::now() - desire_duration);
        let duration = self.render_at.checked_duration_since(Instant::now()).unwrap_or(Duration::from_millis(1));
        ctx.run_later(duration, Self::render_loop);
    }

    fn run_script(&mut self, src: &str, ctx: &mut <Self as Actor>::Context) {
        let mut rt = Runtime::new();
        rt.insert("framerate", r(FRAMERATE_DEFAULT as i32));
        rt.insert("samplerate", r(SAMPLERATE_DEFAULT as i32));
        rt.insert("frame_size", r(vec![r(600 as i32), r(400 as i32)]));
        rt.insert("start_frame", r(0i32));
        rt.insert("end_frame", r(false));
        rt.insert("loop", r(false));
        rt.insert("frame_height", r(400 as i32));
        rt.insert("__rt_cache", self.rt_cache.clone());
        match rt.re(&src) {
            Ok(_) => {
                self.render = rt.get("video").and_then(|val| val.downcast_ref::<Rc<dyn Render<Rgba>>>().cloned());
                self.audio_render = rt.get("audio").and_then(|val| val.downcast_ref::<Rc<dyn AudioRender>>().cloned());
                if let Some(val) = rt.get("framerate") {
                    let framerate = *val.downcast_ref::<i32>().unwrap();
                    self.framerate = framerate.min(120).max(1) as usize;
                }
                if let Some(val) = rt.get("samplerate") {
                    let samplerate = *val.downcast_ref::<i32>().unwrap();
                    self.samplerate = samplerate.min(48000).max(4000) as usize;
                }
                if let Some(val) = rt.get("frame_size") {
                    let val = val;
                    let vec = val.downcast_ref::<Vec<Val>>().unwrap();
                    let width = *vec[0].downcast_ref::<i32>().unwrap();
                    let height = *vec[1].downcast_ref::<i32>().unwrap();
                    self.size = (width.max(0) as usize, height.max(0) as usize);
                }
                self.start_frame = rt.get("start_frame").and_then(|val| val.downcast_ref::<i32>().copied()).unwrap_or(0);
                self.end_frame = rt.get("end_frame").and_then(|val| val.downcast_ref::<i32>().copied()).map(|val| val.max(1));
                self.loop_ = rt.get("loop").and_then(|val| val.downcast_ref::<bool>().copied()).unwrap_or(false);
                self.current_frame = Some(self.start_frame);
                let channel_num = self.audio_render.as_ref().map(|r| r.channel_num()).unwrap_or(0);
                ctx.text(format!(r#"{{"type":"streamInfo","framerate":{:?},"samplerate":{:?},"channelNum":{:?}}}"#, self.framerate, self.samplerate, channel_num));
            },
            Err(mes) => ctx.text(format!(r#"{{"type":"parseFailed","error":{:?}}}"#, format!("{}", mes)))
        }
    }
}
