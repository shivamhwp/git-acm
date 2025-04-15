use isahc::{prelude::*, Request};
use serde_json::{json, Value};
use std::time::Duration;
use yansi::Paint;

use crate::utils::checks::Check;
use crate::utils::config::{get_api_key, load_model_from_pref   };
use crate::utils::diff::get_diff;

pub fn gemini() -> String {
    //checks if env exists
    dotenvy::dotenv().ok();
    
    let model = load_model_from_pref(Some("gemini"));

    let api_url = format!("https://generativelanguage.googleapis.com/v1beta/models/{}:generateContent", model);

    let api_key = get_api_key("gemini");

    Check::api_key_present(&api_key);
    Check::api_url_present(&api_url);


    let prompt = include_str!("../../assets/prompt.txt");
    let full_diff = get_diff();

    Check::is_prompt_empty(prompt);

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
        .timeout(Duration::from_secs(15))
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
