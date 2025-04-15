use isahc::{prelude::*, Request};
use serde_json::{json, Value};
use std::time::Duration;
use yansi::Paint;

use crate::utils::checks::Check;
use crate::utils::config::{get_api_key, load_model_from_pref};
use crate::utils::diff::get_diff;

pub fn anthropic() -> String {
    //checks if env exists
    dotenvy::dotenv().ok();
    let api_url = "https://api.anthropic.com/v1/messages";
    let api_key = get_api_key("anthropic");
    Check::api_key_present(&api_key);
    Check::api_url_present(&api_url);

    let prompt = include_str!("../../assets/prompt.txt");
    let full_diff = get_diff();

    Check::is_prompt_empty(prompt);

    let uri = format!("{}?key={}", api_url, api_key);

    // this will always load anthropic model from the config file, coz it's only called when the model_provider is anthropic.
    let model = load_model_from_pref(Some("anthropic"));


    let req_body = json!({
    "model": model,
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
                return clear_msg.to_string();
            }
            Err(e) => {
                println!("{}", e.red());
                return "".to_string();
            }
        },
        Err(e) => {
            println!("{}", e.red());
            return "".to_string();
        }
    }
}
