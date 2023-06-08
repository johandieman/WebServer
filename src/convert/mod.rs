

// use actix_web::{post, Responder, HttpResponse};
use std::collections::HashMap;
use reqwest::header::{HeaderValue, HeaderMap};

mod transcode;

// curl https://api.openai.com/v1/audio/transcriptions \
//   -H "Authorization: Bearer $OPENAI_API_KEY" \
//   -H "Content-Type: multipart/form-data" \
//   -F file="@/path/to/file/audio.mp3" \
//   -F model="whisper-1"


// #[post("/convert")]
pub async fn convert() -> Result<(), Box<dyn std::error::Error>> {

    let client = reqwest::Client::new();
    let mut map = HashMap::new();
    let headers = HeaderMap::new();

    let myFile = transcode::convert(Some("./test.mp4".to_string()),Some("anull".to_string()),Some(3000));
    
    match myFile  {
        Ok(file)=>{
            map.insert("file", &file);        
        },
        Err(err)=> return Err(Box::new(err)),
    }


    headers.insert("Authorization", HeaderValue::from_static("Bearer "));
    headers.insert("Content-Type", HeaderValue::from_static("multipart/form-data"));
    map.insert("model", "whisper-1");

    let resp = client.post("https://api.openai.com/v1/audio/transcriptions").headers(headers).json(&map).send().await?;


    println!("{:#?}", resp);

    Ok(())



}
