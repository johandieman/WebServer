extern crate ffmpeg_next as ffmpeg;



// use reqwest::header::{HeaderValue, HeaderMap};

mod transcribe;
mod transcode;

// #[pyo3_asyncio::async_std::main]
#[tokio::main]
async fn main() {
    

    let my_file = transcode::convert(Some("./video.mp4".to_string()),Some("anull".to_string()),Some(3000));
    let arr = match my_file  {
        Ok(file)=> file,
        Err(err)=> panic!("Error: {:?}", err),
    };

    transcribe::edit(arr).unwrap();


    // let client = reqwest::Client::new();
    // let mut headers = HeaderMap::new();

    


    // headers.insert("Authorization", HeaderValue::from_static("Bearer "));
    // headers.insert("Content-Type", HeaderValue::from_static("multipart/form-data"));
    // // map.insert("model", "whisper-1");

    // let resp = client.post("https://api.openai.com/v1/audio/transcriptions").headers(headers).json(&map).send().await;

    
    // println!("{:#?}", resp);

}
