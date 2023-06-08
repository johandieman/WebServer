use actix_web::{get, App, HttpResponse, HttpServer, Responder};
// use std::env;
mod convert;


#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("alive")
}

#[actix_web::main]
async fn main()
//  -> std::io::Result<()> 
{
    // let default:u16 = 8080;
    // // let port:u16 = env::var("PORT").unwrap().parse::<u16>().unwrap_or(default);
    // HttpServer::new(|| {
    //     App::new().service(hello).service(convert::convert)
    // })
    // .bind(("127.0.0.1",default))?
    // .run()
    // .await

    convert::convert().await;

}