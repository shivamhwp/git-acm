use isahc::{prelude::*, Request};
use serde_json::{json, Value};
use std::env;
use std::time::Duration;
use yansi::Paint;

use crate::utils::diff::get_diff;

pub fn anthropic() {
    //checks if env exists
    dotenvy::dotenv().ok();
    let api_key = env::var("ANTHROPIC_API_KEY").expect("API_KEY must be set");
    let api_url = env::var("ANTHROPIC_API_URL").expect("API_URL must be set");

    let prompt = include_str!("../../assets/prompt.txt");

    if prompt.is_empty() {
        println!("{}", "no prompt found".red());
        return;
    }

    let full_diff = get_diff();

    let uri = format!("{}?key={}", api_url, api_key);

    let req_body = json!({
    "model": "claude-3-5-haiku-20241022",
    "max_tokens": 300,
    "system": [
       {
        "type": "text",
        "text": prompt,
        "cache_control": {"type": "ephemeral"}
    }
    ],
    "messages": [
        {"role": "user", "content": format!("here's the git diff {}", full_diff) }
    ]
     });

    let response = Request::post(uri)
        .header("x-api-key", api_key)
        .header("anthropic-version", "2023-06-01")
        .header("content-type", "application/json")
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
                let commit_msg = &v["content"][0]["text"];
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
