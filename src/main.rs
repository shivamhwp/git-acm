use isahc::{prelude::*, Request};
use serde_json::{json, Value};
use std::time::Duration;
use std::{env, fs};
mod diff;

fn main() {
    get_commit_msg();
}

fn get_commit_msg() {
    dotenvy::dotenv().unwrap();
    let api_key = env::var("GEMINI_API_KEY").expect("API_KEY must be set");

    let prompt = fs::read_to_string("src/prompt.txt").expect("error occured");

    let full_diff = diff::get_diff();

    let uri = format!("https://generativelanguage.googleapis.com/v1beta/models/gemini-1.5-flash:generateContent?key={}", api_key);

    let req_body = json!({
    "tools": [{"code_execution": {}}],
    "contents": [{
    "parts":[
    {
    "text": format!("{:?}{}", prompt , full_diff)
        }
     ]
          }] ,
         });

    let response = Request::post(uri)
        .header("Content-Type", "application/json")
        .timeout(Duration::from_secs(10))
        .body(req_body.to_string())
        .unwrap()
        .send();

    match response {
        Ok(mut res) => match res.text() {
            Ok(res) => {
                // if you wanna more fields like promptTokenCount, totalTokenCount etc.
                // println!("{}", v) > you'll get the field names > access them using serde_json's get method
                // https://docs.rs/serde_json/latest/serde_json/enum.Value.html#method.get

                let v: Value = serde_json::from_str(&res).unwrap();
                let commit_msg = &v["candidates"][0]["content"]["parts"][0]["text"];

                let final_msg = commit_msg.to_string();
                let clear_msg = final_msg.trim_end();
                println!("{}", clear_msg);
            }
            Err(e) => {
                println!("{}", e)
            }
        },
        Err(e) => {
            println!("{}", e)
        }
    }
}
