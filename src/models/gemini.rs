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
    if full_diff.is_empty() {
        println!(
            "{}",
            "either there are no changes or i'm unable to find diff for some reason.".red()
        );
        println!("{}", "💡 try `git add <file_name> `".red());
        return;
    }

    let uri = format!("{}?key={}", api_url, api_key);

    let req_body = json!({
      "tools": [],
      "systemInstruction": {
      "parts": [
        {
          "text": prompt
        }
      ]
    },
      "contents": [{
          "parts": [
              {
                  "text": full_diff
              }
          ],
          "role": "User"
      }]
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
                let v: Value = serde_json::from_str(&res).unwrap();
                let commit_msg = &v["candidates"][0]["content"]["parts"][0]["text"];

                let final_msg = commit_msg.to_string();
                let clear_msg = final_msg.trim().trim_matches(|c| c == '"' || c == '\n');
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
