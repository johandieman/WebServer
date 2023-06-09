


use std::collections::HashMap;
use reqwest::header::{HeaderValue, HeaderMap};

mod transcode;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

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

    let resp = client.post("https://api.openai.com/v1/audio/transcriptions").headers(headers).json(&map).send().await;


    println!("{:#?}", resp);

    Ok(())
}
