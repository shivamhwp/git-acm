use isahc::{prelude::*, Request};
use serde_json::{json, Value};
use std::env;
use std::time::Duration;
use yansi::Paint;

use crate::utils::diff::get_diff;

pub fn openai() {
    //checks if env exists
    dotenvy::dotenv().ok();
    let api_key = env::var("OPENAI_API_KEY").expect("API_KEY must be set");
    let api_url = env::var("OPENAI_API_URL").expect("API_URL must be set");

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
