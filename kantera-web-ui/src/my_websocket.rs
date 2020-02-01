use std::time::{Duration, Instant};

use actix::prelude::*;
use actix_web_actors::ws;

use kantera::{
    pixel::Rgba,
    render::{Render, Dummy, RenderOpt, Range},
    export::render_to_buffer,
    script::{Runtime, r, Val}
};
use std::rc::Rc;


const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);
const FRAMERATE_DEFAULT: usize = 30;

pub struct MyWebSocket {
    hb: Instant,
    frame: i32,
    render: Rc<dyn Render<Rgba>>,
    framerate: usize,
    size: (usize, usize),
    render_at: Instant
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
                rt.insert("frame_size", r(vec![r(600 as i32), r(400 as i32)]));
                rt.insert("frame_height", r(400 as i32));
                match rt.re(&text) {
                    Ok(val) => {
                        self.render = val.borrow().downcast_ref::<Rc<dyn Render<Rgba>>>().unwrap().clone();
                        if let Some(val) = rt.get("framerate") {
                            let framerate = *val.borrow().downcast_ref::<i32>().unwrap();
                            self.framerate = framerate.min(120).max(1) as usize;
                        }
                        if let Some(val) = rt.get("frame_size") {
                            let val = val.borrow();
                            let vec = val.downcast_ref::<Vec<Val>>().unwrap();
                            let width = *vec[0].borrow().downcast_ref::<i32>().unwrap();
                            let height = *vec[1].borrow().downcast_ref::<i32>().unwrap();
                            self.size = (width.max(0) as usize, height.max(0) as usize);
                        }
                        self.frame = 0;
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
            framerate: 30,
            size: (600, 400),
            render_at: Instant::now()
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
        let mut vec = Vec::new();
        image::png::PNGEncoder::new(&mut vec).encode(&buf, width as u32, height as u32, image::RGBA(8)).unwrap();
        ctx.binary(vec);
        ctx.text(format!(r#"{{"type":"sync","frame":{}}}"#, frame));
        self.frame += 1;

        let desire_duration = Duration::from_millis(1000 / self.framerate as u64);
        self.render_at = (self.render_at + desire_duration).max(Instant::now() - desire_duration);
        let duration = self.render_at.checked_duration_since(Instant::now()).unwrap_or(Duration::from_millis(1));
        ctx.run_later(duration, Self::render_loop);
    }
}
