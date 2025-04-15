use isahc::{prelude::*, Request};
use serde_json::{json, Value};
use std::time::Duration;
use yansi::Paint;

use crate::utils::checks::Check;
use crate::utils::config::{get_api_url, load_model_from_pref};
use crate::utils::diff::get_diff;

pub fn llama() -> String {
    //checks if env exists
    dotenvy::dotenv().ok();
    let api_url = get_api_url("llama", "");
    let model_name = load_model_from_pref(Some("llama"));

    Check::api_url_present(&api_url);
    Check::is_model_name_empty(&model_name);

    let prompt = include_str!("../../assets/prompt.txt");
    let full_diff = get_diff();

    Check::is_prompt_empty(prompt);

    let uri = format!("{}", api_url);

    let req_body = json!({
    "model": model_name,
    "system" : prompt,
    "prompt": full_diff,
    "stream": false
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
                let commit_msg = &v["response"];

                let final_msg = commit_msg.to_string();
                let clear_msg = final_msg.trim().trim_matches(|c| c == '"' || c == '\n');
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
