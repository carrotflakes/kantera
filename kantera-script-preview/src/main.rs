#[macro_use]
extern crate actix_web;

mod my_websocket;
mod rendering_engine;

use std::path::PathBuf;

use actix::prelude::*;
use actix_cors::Cors;
use actix_web::{
    http::StatusCode, middleware, web, App, Error, HttpRequest, HttpResponse, HttpServer,
};
use actix_web_actors::ws;

use my_websocket::MyWebSocket;
use rendering_engine::RenderingEngine;

#[get("/ws")]
async fn ws_index(
    status: web::Data<Addr<RenderingEngine>>,
    r: HttpRequest,
    stream: web::Payload,
) -> Result<HttpResponse, Error> {
    let re = status.as_ref().clone();
    println!("{:?}", r);
    let res = ws::start(MyWebSocket::new(re), &r, stream);
    println!("{:?}", res);
    res
}

#[get("/")]
async fn index() -> actix_web::Result<HttpResponse> {
    Ok(HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body(include_str!("../static/index.html")))
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    #[cfg(debug_assertions)]
    println!(
        "\x1b[31mThis is built without optimization! Kantera recommends release build.\x1b[0m"
    );

    let directory_path = PathBuf::from(
        std::env::args()
            .collect::<Vec<String>>()
            .get(1)
            .unwrap_or(&"".to_string()),
    );
    let rendering_engine = RenderingEngine::new(directory_path).start();

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
            .service(ws_index)
            .service(index)
        //.service(fs::Files::new("/", "static/").index_file("index.html"))
    })
    .bind(addr)?
    .run()
    .await
}
