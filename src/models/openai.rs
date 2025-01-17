use isahc::{prelude::*, Request};
use serde_json::{json, Value};
use std::time::Duration;
use yansi::Paint;

use crate::utils::checks::Check;
use crate::utils::config::{copy_to_clipboard, get_api_key, get_api_url, run_git_commit};
use crate::utils::diff::get_diff;

pub fn openai() {
    //checks if env exists
    dotenvy::dotenv().ok();
    let api_url = get_api_url("openai", "https://api.openai.com/v1/chat/completions");
    let api_key = get_api_key("openai");

    Check::api_key_present(&api_key);
    Check::api_url_present(&api_url);

    let prompt = include_str!("../../assets/prompt.txt");
    let full_diff = get_diff();

    Check::is_prompt_empty(prompt);
    Check::is_diff_empty(&full_diff);
    let uri = format!("{}?key={}", api_url, api_key);

    let req_body = json!({
        "model": "gpt-4o",
        "messages": [
            {
                "role": "developer",
                "content": [
                    {
                        "type": "text",
                        "text": prompt
                    }
                ]
            },
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": format!("here's the git diff from which you have to generate a git-commit-message {}", full_diff)
                    }
                ]
            }
        ]
    });

    let response = Request::post(uri)
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", api_key))
        .timeout(Duration::from_secs(10))
        .body(req_body.to_string())
        .unwrap()
        .send();

    match response {
        Ok(mut res) => match res.text() {
            Ok(res) => {
                let v: Value = serde_json::from_str(&res).unwrap();
                let commit_msg = &v["choices"][0]["message"]["content"];

                let final_msg = commit_msg.to_string();
                let clear_msg = final_msg.trim_matches(|c| c == '"' || c == '\n');
                println!("{}", clear_msg.blue());
                copy_to_clipboard(clear_msg).unwrap_or_default();
                run_git_commit(clear_msg);
            }
            Err(e) => {
                println!("{}", e.red());
            }
        },
        Err(e) => {
            println!("{}", e.red());
        }
    }
    return;
}
