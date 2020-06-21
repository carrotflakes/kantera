mod my_websocket;
mod rendering_engine;

use std::path::PathBuf;
use std::sync::{Arc, Mutex};

use actix_cors::Cors;
use actix_files as fs;
use actix_web::{middleware, web, App, Error, HttpRequest, HttpResponse, HttpServer};
use actix_web_actors::ws;

use my_websocket::MyWebSocket;
use rendering_engine::{RRenderingEngine, RenderingEngine};

async fn ws_index(status: web::Data<RRenderingEngine>, r: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    let re = status.as_ref().clone();
    println!("{:?}", r);
    let res = ws::start(MyWebSocket::new(re), &r, stream);
    println!("{:?}", res);
    res
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    #[cfg(debug_assertions)]
    println!(
        "\x1b[31mThis is built without optimization! Kantera recommends release build.\x1b[0m"
    );

    let directory_path = PathBuf::from("./workspace");
    let rendering_engine = Arc::new(Mutex::new(RenderingEngine::new(directory_path)));
    RenderingEngine::start(rendering_engine.clone());

    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    let data = web::Data::new(rendering_engine);

    let addr = format!(
        "0.0.0.0:{}",
        std::env::var("PORT").unwrap_or("8080".to_string())
    );
    println!("addr: {}", addr);
    HttpServer::new(move || {
        App::new()
            .app_data(data.clone())
            .wrap(
                Cors::new()
                    .allowed_methods(vec!["GET", "POST"])
                    .max_age(3600)
                    .finish(),
            )
            .wrap(middleware::Logger::default())
            .service(web::resource("/ws/").route(web::get().to(ws_index)))
            .service(fs::Files::new("/", "static/").index_file("index.html"))
    })
    .bind(addr)?
    .run()
    .await
}
