use openrouter_rs::{api::chat::*, types::Role, OpenRouterClient};
use yansi::Paint;

use crate::utils::checks::Check;
use crate::utils::config::{get_api_key, save_models_list, StoredModel};
use openrouter_rs::api::models;

// Removed custom response structs in favor of openrouter_rs models API
use crate::utils::diff::get_diff;

pub fn build_openrouter_client() -> Result<OpenRouterClient, Box<dyn std::error::Error>> {
    let api_key = get_api_key();
    Check::api_key_present(&api_key);
    match OpenRouterClient::builder().api_key(&api_key).build() {
        Ok(client) => Ok(client),
        Err(e) => Err(format!("Failed to create OpenRouter client: {}", e).into()),
    }
}

pub async fn get_commit_message_from_openrouter(
    client: &OpenRouterClient,
    model: &str,
) -> String {
    let full_diff = get_diff();

    let prompt = include_str!("../../assets/prompt.txt");

    Check::is_prompt_empty(prompt);

    let user_prompt = format!(
        "here's the git diff from which you have to generate a git-commit-message {}",
        full_diff
    );

    // Create chat completion request
    let request = match ChatCompletionRequest::builder()
        .model(model)
        .messages(vec![
            Message::new(Role::System, prompt),
            Message::new(Role::User, &user_prompt),
        ])
        .build()
    {
        Ok(req) => req,
        Err(e) => {
            println!("{}", format!("Failed to create request: {}", e).red());
            return String::new();
        }
    };

    // Send request
    match client.send_chat_completion(&request).await {
        Ok(response) => {
            if let Some(content) = response.choices.get(0).and_then(|c| c.content()) {
                let clear_msg = content.trim_matches(|c| c == '"' || c == '\n');
                return clear_msg.to_string();
            }
            println!("{}", "No response content received".red());
            String::new()
        }
        Err(e) => {
            println!("{}", format!("API request failed: {}", e).red());
            String::new()
        }
    }
}

pub async fn fetch_and_store_models() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = get_api_key();
    Check::api_key_present(&api_key);

    // Use the official SDK to list models
    let api_models =
        models::list_models("https://openrouter.ai/api/v1", &api_key, None, None).await?;

    // Map SDK models to our stored representation
    let mut models: Vec<StoredModel> = api_models
        .into_iter()
        .map(|m| StoredModel {
            name: m.name,
            // Canonical slug corresponds to the model id in OpenRouter
            canonical_slug: m.id,
        })
        .collect();

    models.sort_by_key(|m| m.name.to_lowercase());

    save_models_list(&models)?;
    println!("{} {}", "fetched models:".green(), models.len());
    Ok(())
}
