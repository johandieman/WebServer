
use actix_web::{post};

mod transcode;

#[post("/")]
pub fn convert() -> impl Responder{
    HttpResponse::Ok().body("alive")
}
