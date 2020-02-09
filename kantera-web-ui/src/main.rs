mod my_websocket;

use std::io::Write;

use actix_web::{middleware, web, App, Error, HttpRequest, HttpResponse, HttpServer};
use actix_files as fs;
use actix_web_actors::ws;
use actix_multipart::Multipart;
use futures::StreamExt;

use my_websocket::MyWebSocket;

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
async fn main() -> std::io::Result<()> {
    unsafe {
        kantera::export::DEBUG_PRINT = false;
    }
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    std::fs::create_dir_all("./tmp").unwrap();

    let addr = format!("0.0.0.0:{}", std::env::var("PORT").unwrap_or("8080".to_string()));
    println!("addr: {}", addr);
    HttpServer::new(move || {
        App::new()
        .wrap(middleware::Logger::default())
        .service(web::resource("/ws/").route(web::get().to(ws_index)))
        .service(web::resource("/upload").route(web::post().to(save_file)))
        .service(fs::Files::new("/", "static/").index_file("index.html"))
    })
    .bind(addr)?
    .run()
    .await
}
