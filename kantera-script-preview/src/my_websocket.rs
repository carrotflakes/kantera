use std::time::{Duration, Instant};

use actix::prelude::*;
use actix_web_actors::ws;

use crate::rendering_engine::{Frame, RenderingEngine, Subscribe};

const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

pub struct MyWebSocket {
    hb: Instant,
    re: Addr<RenderingEngine>,
}

impl Actor for MyWebSocket {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.haertbeat(ctx);
        self.re
            .do_send(Subscribe::new(ctx.address().recipient::<Frame>()));
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
            Ok(ws::Message::Binary(_)) => {}
            Ok(ws::Message::Close(_)) => ctx.stop(),
            _ => ctx.stop(),
        }
    }
}

impl MyWebSocket {
    pub fn new(re: Addr<RenderingEngine>) -> Self {
        Self {
            hb: Instant::now(),
            re,
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
}

impl Handler<Frame> for MyWebSocket {
    type Result = ();

    fn handle(&mut self, msg: Frame, ctx: &mut Self::Context) -> Self::Result {
        if let Some(data) = msg.video {
            ctx.text(r#"{"type":"frame"}"#);
            ctx.binary(data);
        }
        if let Some(data) = msg.audio {
            ctx.text(format!(
                r#"{{"type":"audio","samplerate":{}}}"#,
                msg.samplerate
            ));
            ctx.binary(data);
        }

        ctx.text(format!(
            r#"{{"type":"sync","frame":{}}}"#,
            msg.current_frame
        ));
    }
}
