extern crate ffmpeg_next as ffmpeg;


use std::collections::HashMap;
use reqwest::header::{HeaderValue, HeaderMap};

mod transcode;

#[tokio::main]
async fn main() {

    let client = reqwest::Client::new();
    let mut map = HashMap::new();
    let mut headers = HeaderMap::new();

    let my_file = transcode::convert(Some("./test.mp4".to_string()),Some("anull".to_string()),Some(3000));
    
    match my_file  {
        Ok(file)=>{
            map.insert("file", file);        
        },
        Err(err)=> panic!("Error: {:?}", err),
        
    }


    headers.insert("Authorization", HeaderValue::from_static("Bearer "));
    headers.insert("Content-Type", HeaderValue::from_static("multipart/form-data"));
    // map.insert("model", "whisper-1");

    let resp = client.post("https://api.openai.com/v1/audio/transcriptions").headers(headers).json(&map).send().await;


    // println!("{:#?}", resp);

}
