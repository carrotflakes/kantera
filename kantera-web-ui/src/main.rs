extern crate image;
use std::io;
use std::sync::Mutex;
use std::time::{Duration, Instant};
use std::io::Write;

use actix::prelude::*;
use actix_web::{middleware, web, App, Error, HttpRequest, HttpResponse, HttpServer};
use actix_files as fs;
use actix_web_actors::ws;
use actix_multipart::Multipart;
use futures::StreamExt;

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
                        self.render = res.borrow().downcast_ref::<Rc<dyn Render<Rgba>>>().unwrap().clone();
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
        self.render_at = (self.render_at + desire_duration).max(Instant::now() - desire_duration);
        let duration = self.render_at.checked_duration_since(Instant::now()).unwrap_or(Duration::from_millis(1));
        ctx.run_later(duration, Self::render_loop);
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

async fn save_file(mut payload: Multipart) -> Result<HttpResponse, Error> {
    let mut filepathes = Vec::new();
    // iterate over multipart stream
    while let Some(item) = payload.next().await {
        let mut field = item?;
        let content_type = field.content_disposition().unwrap();
        let filename = content_type.get_filename().unwrap();
        let filepath = format!("./tmp/{}", filename);
        filepathes.push(format!("{:?}", filepath.clone()));
        // File::create is blocking operation, use threadpool
        let mut f = web::block(|| std::fs::File::create(filepath))
            .await
            .unwrap();
        // Field in turn is stream of *Bytes* object
        while let Some(chunk) = field.next().await {
            let data = chunk.unwrap();
            // filesystem operations are blocking, we have to use threadpool
            f = web::block(move || f.write_all(&data).map(|_| f)).await?;
        }
    }
    Ok(HttpResponse::Ok().body(format!("[{}]", filepathes.join(","))))
}

#[actix_rt::main]
async fn main() -> io::Result<()> {
    unsafe {
        DEBUG_PRINT = false;
    }
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    std::fs::create_dir_all("./tmp").unwrap();

    let data = web::Data::new(Mutex::new(0usize));

    HttpServer::new(move || {
        App::new()
        .app_data(data.clone())
        .wrap(middleware::Logger::default())
        .service(web::resource("/").to(index))
        .service(web::resource("/ws/").route(web::get().to(ws_index)))
        .service(web::resource("/upload").route(web::post().to(save_file)))
        .service(fs::Files::new("/", "static/").index_file("index.html"))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
