
use actix_web::{post, Responder, HttpResponse};

mod transcode;

// #[post("/convert")]
pub async fn convert() 
// -> impl Responder 
{
    // HttpResponse::Ok().body("alive")

    transcode::convert();
}
