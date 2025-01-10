use isahc::{prelude::*, Request};
use serde_json::{json, Value};
use std::env;
use std::time::Duration;
use yansi::Paint;

use crate::utils::diff::get_diff;

pub fn gemini() {
    //checks if env exists
    dotenvy::dotenv().ok();
    let api_key = env::var("GEMINI_API_KEY").expect("API_KEY must be set");
    let api_url = env::var("GEMINI_API_URL").expect("API_URL must be set");

    let prompt = include_str!("../../assets/prompt.txt");

    if prompt.is_empty() {
        println!("{}", "no prompt found".red());
        return;
    }

    let full_diff = get_diff();

    let uri = format!("{}?key={}", api_url, api_key);

    let req_body = json!({
    "tools": [{"code_execution": {}}],
    "contents": [{
    "parts":[
    {
    "text": format!("{:?}{:?}", prompt , full_diff)
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
                println!("{}", clear_msg.blue());
            }
            Err(e) => {
                println!("{}", e.red())
            }
        },
        Err(e) => {
            println!("{}", e.red())
        }
    }
    return;
}
