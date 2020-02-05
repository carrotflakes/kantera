use std::time::{Duration, Instant};

use actix::prelude::*;
use actix_web_actors::ws;

use kantera::{
    pixel::Rgba,
    render::{Render, Dummy, RenderOpt, Range},
    audio_render::AudioRender,
    export::render_to_buffer,
    script::{Runtime, r, Val}
};
use std::rc::Rc;
use std::collections::HashMap;


const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);
const FRAMERATE_DEFAULT: usize = 30;
const SAMPLERATE_DEFAULT: usize = 16000;

pub struct MyWebSocket {
    hb: Instant,
    frame: i32,
    render: Rc<dyn Render<Rgba>>,
    audio_render: Option<Rc<dyn AudioRender>>,
    framerate: usize,
    samplerate: usize,
    frame_num: Option<usize>,
    size: (usize, usize),
    render_at: Instant,
    rt_cache: Val
}

impl Actor for MyWebSocket {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.haertbeat(ctx);
        self.render_loop(ctx);
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for MyWebSocket {
    fn handle(
        &mut self,
        msg: Result<ws::Message, ws::ProtocolError>,
        ctx: &mut Self::Context,
    ) {
        println!("WS: {:?}", msg);
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
                let mut rt = Runtime::new();
                rt.insert("framerate", r(FRAMERATE_DEFAULT as i32));
                rt.insert("samplerate", r(SAMPLERATE_DEFAULT as i32));
                rt.insert("frame_size", r(vec![r(600 as i32), r(400 as i32)]));
                rt.insert("frame_num", r(false));
                rt.insert("frame_height", r(400 as i32));
                rt.insert("__rt_cache", self.rt_cache.clone());
                match rt.re(&text) {
                    Ok(val) => {
                        self.render = val.borrow().downcast_ref::<Rc<dyn Render<Rgba>>>().unwrap().clone();
                        if let Some(val) = rt.get("framerate") {
                            let framerate = *val.borrow().downcast_ref::<i32>().unwrap();
                            self.framerate = framerate.min(120).max(1) as usize;
                        }
                        if let Some(val) = rt.get("samplerate") {
                            let samplerate = *val.borrow().downcast_ref::<i32>().unwrap();
                            self.samplerate = samplerate.min(48000).max(4000) as usize;
                        }
                        if let Some(val) = rt.get("frame_size") {
                            let val = val.borrow();
                            let vec = val.downcast_ref::<Vec<Val>>().unwrap();
                            let width = *vec[0].borrow().downcast_ref::<i32>().unwrap();
                            let height = *vec[1].borrow().downcast_ref::<i32>().unwrap();
                            self.size = (width.max(0) as usize, height.max(0) as usize);
                        }
                        if let Some(val) = rt.get("audio") {
                            let audio_render = val.borrow().downcast_ref::<Rc<dyn AudioRender>>().unwrap().clone();
                            self.audio_render = Some(audio_render);
                        } else {
                            self.audio_render = None;
                        }
                        self.frame_num = rt.get("frame_num").and_then(|val| val.borrow().downcast_ref::<i32>().copied()).map(|val| val.max(1) as usize);
                        self.frame = 0;
                        ctx.text(format!(r#"{{"type":"streamInfo","framerate":{:?},"samplerate":{:?}}}"#, self.framerate, self.samplerate));
                    },
                    Err(mes) => ctx.text(format!(r#"{{"type":"parseFailed","error":{:?}}}"#, mes))
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
            frame: 0,
            render: Rc::new(Dummy()),
            audio_render: None,
            framerate: FRAMERATE_DEFAULT,
            samplerate: SAMPLERATE_DEFAULT,
            frame_num: None,
            size: (600, 400),
            render_at: Instant::now(),
            rt_cache: r(HashMap::<String, Val>::new())
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
        let frame = self.frame;
        let (width, height) = self.size;
        let buffer = render_to_buffer(&RenderOpt {
            u_range: Range::unit(),
            u_res: width,
            v_range: Range::unit(),
            v_res: height,
            frame_range: frame..frame+1,
            framerate: self.framerate
        }, &self.render);
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
        self.frame += 1;
        if let Some(frame_num) = self.frame_num {
            if frame_num as i32 <= self.frame {
                self.frame = 0;
            }
        }

        let desire_duration = Duration::from_millis(1000 / self.framerate as u64);
        self.render_at = (self.render_at + desire_duration).max(Instant::now() - desire_duration);
        let duration = self.render_at.checked_duration_since(Instant::now()).unwrap_or(Duration::from_millis(1));
        ctx.run_later(duration, Self::render_loop);
    }
}
