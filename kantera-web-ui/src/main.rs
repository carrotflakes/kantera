extern crate image;
use std::io;
use std::sync::Mutex;
use std::time::{Duration, Instant};

use actix::prelude::*;
use actix_web::{middleware, web, App, Error, HttpRequest, HttpResponse, HttpServer};
use actix_files as fs;
use actix_web_actors::ws;

use kantera::{
    pixel::Rgba,
    render::{Render, Dummy, RenderOpt, Range},
    export::{render_to_buffer, DEBUG_PRINT},
    script::*
};
use std::rc::Rc;

const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

struct MyWebSocket {
    hb: Instant,
    frame: Mutex<i32>,
    render: Rc<dyn Render<Rgba>>,
    framerate: usize,
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
                let mut reader = make_reader();
                match reader.parse_top_level(&text) {
                    Ok(mut forms) => {
                        forms.insert(0, reader.parse("do").unwrap());
                        let res = eval(make_env(), r(forms));
                        self.render = res.borrow().downcast_ref::<Option<Rc<dyn Render<Rgba>>>>().unwrap().as_ref().unwrap().clone();
                        *self.frame.lock().unwrap() = 0;
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
    fn new() -> Self {
        Self {
            hb: Instant::now(),
            frame: Mutex::new(0),
            render: Rc::new(Dummy()),
            framerate: 30,
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
        let (width, height) = (600usize, 400usize);
        let frame = {
            let mut frame = self.frame.lock().unwrap();
            *frame += 1;
            *frame - 1
        };
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

        let desire_duration = Duration::from_millis(1000 / self.framerate as u64);
        let delay = Instant::now() - self.render_at;
        let duration = if desire_duration > delay {desire_duration - delay} else {Duration::from_millis(0)};
        self.render_at = Instant::now() + duration;
        ctx.run_later(duration, |act, ctx| {
            act.render_loop(ctx)
        });
    }
}

async fn index(state: web::Data<Mutex<usize>>, req: HttpRequest) -> HttpResponse {
    println!("{:?}", req);
    *(state.lock().unwrap()) += 1;

    HttpResponse::Ok().body(format!("Num of requests: {}", state.lock().unwrap()))
}

async fn ws_index(r: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    println!("{:?}", r);
    let res = ws::start(MyWebSocket::new(), &r, stream);
    println!("{:?}", res);
    res
}

#[actix_rt::main]
async fn main() -> io::Result<()> {
    unsafe {
        DEBUG_PRINT = false;
    }
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    let data = web::Data::new(Mutex::new(0usize));

    HttpServer::new(move || {
        App::new()
        .app_data(data.clone())
        .wrap(middleware::Logger::default())
        .service(web::resource("/").to(index))
        .service(web::resource("/ws/").route(web::get().to(ws_index)))
        .service(fs::Files::new("/", "static/").index_file("index.html"))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
