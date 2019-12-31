extern crate image;
use std::io;
use std::sync::Mutex;
use std::time::{Duration, Instant};

use actix::prelude::*;
use actix_web::{middleware, web, App, Error, HttpRequest, HttpResponse, HttpServer};
use actix_files as fs;
use actix_web_actors::ws;

use image::{ImageBuffer, RgbImage};

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
    render: Rc<dyn Render<Rgba>>
}

impl Actor for MyWebSocket {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.haertbeat(ctx);
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
            render: Rc::new(Dummy())
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
        let framerate = 30usize;
        ctx.run_interval(Duration::from_millis(1000 / framerate as u64), move |act, ctx| {
            let (width, height) = (600usize, 400usize);
            let frame = {
                let mut frame = act.frame.lock().unwrap();
                *frame += 1;
                *frame - 1
            };
            // let img: RgbImage = ImageBuffer::from_fn(width, height, |x, y| {
            //     image::Rgb([((x + i) % 64) as u8, (x % 128) as u8, (y % 64) as u8])
            // });
            // let mut vec = Vec::new();
            // image::png::PNGEncoder::new(&mut vec).encode(&img.into_raw(), width, height, image::RGB(8)).unwrap();
            let buffer = render_to_buffer(&RenderOpt {
                u_range: Range::unit(),
                u_res: width,
                v_range: Range::unit(),
                v_res: height,
                frame_range: frame..frame+1,
                framerate: framerate
            }, &act.render);
            let buffer: Vec<u8> = buffer.vec.iter().flat_map(|p| vec![p.0, p.1, p.2, p.3]).map(|x| (x * 255.0).floor() as u8).collect();
            let mut vec = Vec::new();
            image::png::PNGEncoder::new(&mut vec).encode(&buffer, width as u32, height as u32, image::RGBA(8)).unwrap();
            ctx.binary(vec);
            ctx.text(format!(r#"{{"type":"sync","frame":{}}}"#, frame));
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
