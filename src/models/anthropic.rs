use isahc::{prelude::*, Request};
use serde_json::{json, Value};
use std::time::Duration;
use yansi::Paint;

use crate::utils::checks::Check;
use crate::utils::config::{copy_to_clipboard, get_api_key, get_api_url};
use crate::utils::diff::get_diff;

pub fn anthropic() {
    //checks if env exists
    dotenvy::dotenv().ok();
    let api_url = get_api_url("anthropic", "https://api.anthropic.com/v1/messages");
    let api_key = get_api_key("anthropic");
    Check::api_key_present(&api_key);
    Check::api_url_present(&api_url);

    let prompt = include_str!("../../assets/prompt.txt");
    let full_diff = get_diff();

    Check::is_prompt_empty(prompt);
    Check::is_diff_empty(&full_diff);

    let uri = format!("{}?key={}", api_url, api_key);

    let req_body = json!({
    "model": "claude-3-5-sonnet-20241022",
    "max_tokens": 60,
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
                let v: Value = serde_json::from_str(&res).unwrap();
                let commit_msg = &v["content"][0]["text"];
                let final_msg = commit_msg.to_string();
                let clear_msg = final_msg.trim_matches(|c| c == '"' || c == '\n');
                println!("{}", clear_msg.blue());
                copy_to_clipboard(clear_msg).unwrap_or_default();
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
