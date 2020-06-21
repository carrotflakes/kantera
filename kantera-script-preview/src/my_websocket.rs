use std::time::{Duration, Instant};

use actix::prelude::*;
use actix_web_actors::ws;

use crate::rendering_engine::RRenderingEngine;

const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

pub struct MyWebSocket {
    hb: Instant,
    framerate: usize,
    render_at: Instant,
    re: RRenderingEngine
}

impl Actor for MyWebSocket {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.haertbeat(ctx);
        self.cast(ctx);
        ctx.text(r#"{"type":"log","log":"ready."}"#);
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for MyWebSocket {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
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
            }
            Ok(ws::Message::Binary(_)) => {},
            Ok(ws::Message::Close(_)) => ctx.stop(),
            _ => ctx.stop(),
        }
    }
}

impl MyWebSocket {
    pub fn new(re: RRenderingEngine) -> Self {
        Self {
            hb: Instant::now(),
            framerate: 30,
            render_at: Instant::now(),
            re
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

    fn cast(&mut self, ctx: &mut <Self as Actor>::Context) {
        {
            let re = self.re.lock().unwrap();
            if let Some(data) = re.get_frame_bin().cloned() {
                ctx.text(r#"{"type":"frame"}"#);
                ctx.binary(data);
            }
            if let Some(data) = re.get_audio_frame_bin().cloned() {
                ctx.text(r#"{"type":"audio"}"#);
                ctx.binary(data);
            }

            ctx.text(format!(r#"{{"type":"sync","frame":{}}}"#, re.get_current_frame()));
        }

        // Schedule next render
        let desire_duration = Duration::from_millis(1000 / self.framerate as u64);
        self.render_at = (self.render_at + desire_duration).max(Instant::now() - desire_duration);
        let duration = self
            .render_at
            .checked_duration_since(Instant::now())
            .unwrap_or(Duration::from_millis(1));
        ctx.run_later(duration, Self::cast);
    }
}
