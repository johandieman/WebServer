use actix_web::{get,web, App, HttpResponse, HttpServer, Responder};
// use std::env;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("alive")
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Server started at localhost:8080")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let default:u16 = 8080;
    // let port:u16 = env::var("PORT").unwrap().parse::<u16>().unwrap_or(default);
    HttpServer::new(|| {
        App::new().service(hello).route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1",default))?
    .run()
    .await
}