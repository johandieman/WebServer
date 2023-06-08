
// use actix_web::{post, Responder, HttpResponse};

mod transcode;

// #[post("/convert")]
pub async fn convert() 
// -> impl Responder 
{

    let myFile = transcode::convert(Some("./test.mp4".to_string()),Some("anull".to_string()),Some(3000));
    
    match myFile  {
        Ok(file)=>println!("{:?}",file),
        Err(err)=>println!("{:?}",err),
    }

    // HttpResponse::Ok().body("alive")
}
