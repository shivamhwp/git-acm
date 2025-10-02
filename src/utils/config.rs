use arboard::Clipboard;
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode},
};
use duct::cmd;
use std::{
    env, fs,
    io::{self, BufReader, BufWriter},
    path::{Path, PathBuf},
};
use yansi::Paint;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Preferences {
    pub default_model: String,
    pub user_selected_model: String,
    pub auto_commit: bool,
}

fn get_config_dir() -> PathBuf {
    let home = env::var("HOME").unwrap_or_else(|_| ".".to_string());
    return Path::new(&home).join(".config").join("git-acm");
}

fn config_exists() -> Result<(), io::Error> {
    let config_dir = get_config_dir();
    if !config_dir.exists() {
        return fs::create_dir(&config_dir);
    }
    Ok(())
}

fn config_file_path() -> PathBuf {
    get_config_dir().join("git-acm-prefs.json")
}

pub fn load_preferences() -> Result<Preferences, Box<dyn std::error::Error>> {
    if let Err(e) = config_exists() {
        println!("{}", format!("Failed to create config dir: {}", e).red());
        let default_prefs = Preferences {
            default_model: "openai/gpt-5-chat".to_string(),
            user_selected_model: "openai/gpt-5-chat".to_string(),
            auto_commit: false,
        };
        return Ok(default_prefs);
    }
    let config_file = config_file_path();
    if !config_file.exists() {
        let default_prefs = Preferences {
            default_model: "openai/gpt-5-chat".to_string(),
            user_selected_model: "openai/gpt-5-chat".to_string(),
            auto_commit: false,
        };
        save_preferences(&default_prefs)?;
        return Ok(default_prefs);
    }
    let file = fs::File::open(&config_file)?;
    let reader = BufReader::new(file);
    let prefs: Preferences = serde_json::from_reader(reader)?;
    Ok(prefs)
}

pub fn save_preferences(prefs: &Preferences) -> Result<(), Box<dyn std::error::Error>> {
    let config_file = config_file_path();
    let file = fs::File::create(&config_file)?;
    let writer = BufWriter::new(file);
    serde_json::to_writer_pretty(writer, prefs)?;
    Ok(())
}

pub fn load_model_from_pref(_provider: Option<&str>) -> String {
    match load_preferences() {
        Ok(prefs) => prefs.user_selected_model,
        Err(_) => "openai/gpt-5-chat".to_string(),
    }
}

pub fn load_auto_commit() -> bool {
    match load_preferences() {
        Ok(prefs) => prefs.auto_commit,
        Err(_) => false,
    }
}

pub fn save_model_value(value: &str) {
    match load_preferences() {
        Ok(mut prefs) => {
            prefs.user_selected_model = value.to_string();
            if let Err(e) = save_preferences(&prefs) {
                println!("{} {}", value, format!(" couldn't save: {}", e).red());
            } else {
                println!("{} {}", value, " saved as default.".green());
            }
        }
        Err(e) => {
            println!(
                "{} {}",
                value,
                format!(" couldn't save, error: {}", e).red()
            );
        }
    }
}

pub fn save_auto_commit(enabled: bool) {
    let status = if enabled { "enabled" } else { "disabled" };
    match load_preferences() {
        Ok(mut prefs) => {
            prefs.auto_commit = enabled;
            if let Err(e) = save_preferences(&prefs) {
                println!(
                    "Autocommit {} failed to save: {}",
                    status,
                    format!("{}", e).red()
                );
            } else {
                println!("autocommit {}", format!("{}", status).green());
            }
        }
        Err(e) => {
            println!("Failed to save autocommit: {}", format!("{}", e).red());
        }
    }
}

pub fn get_api_key() -> String {
    match env::var("OPENROUTER_API_KEY") {
        Ok(k) => k,
        Err(_) => String::new(),
    }
}

pub fn copy_to_clipboard(text: &str) -> Result<(), Box<dyn std::error::Error>> {
    match Clipboard::new()?.set_text(text) {
        Ok(_t) => {
            println!("{}", "copied to clipboard 👍".magenta());
        }
        Err(_e) => {
            println!("{}", "( couldn't copy to clipboard 🥲 )".yellow());
        }
    }
    Ok(())
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct StoredModel {
    pub name: String,
    pub canonical_slug: String,
}

pub fn models_file_path() -> PathBuf {
    get_config_dir().join("models.json")
}

pub fn save_models_list(models: &[StoredModel]) -> Result<(), Box<dyn std::error::Error>> {
    // Ensure config dir exists
    if let Err(e) = config_exists() {
        println!("{}", format!("Failed to create config dir: {}", e).red());
        return Err(Box::new(e));
    }
    let path = models_file_path();
    let file = fs::File::create(&path)?;
    let writer = BufWriter::new(file);
    serde_json::to_writer_pretty(writer, &models)?;
    println!("{} {}", "models saved to".green(), path.display());
    Ok(())
}

pub fn load_models_list() -> Result<Vec<StoredModel>, Box<dyn std::error::Error>> {
    let path = models_file_path();
    if !path.exists() {
        return Err("models.json not found".into());
    }
    let file = fs::File::open(&path)?;
    let reader = BufReader::new(file);
    let models: Vec<StoredModel> = serde_json::from_reader(reader)?;
    Ok(models)
}

pub fn run_git_commit(value: &str) {
    let enabled = load_auto_commit();
    if enabled {
        let err_git_commit_message = "couldn't commit".red().to_string();
        match cmd!("git", "commit", "-m", value.to_string()).read() {
            Ok(_result) => {
                println!("{}.", "committed".magenta());
                println!(
                    "{}",
                    " run `git push` to push the changes to the repo".magenta()
                );
                return;
            }
            Err(e) => {
                println!("{} error : {}", err_git_commit_message, e);
                return;
            }
        }
    } else {
        return;
    }
}

pub fn print_to_cli(value: &str) {
    if value.is_empty() {
        println!("{}", "got no response".red());
        std::process::exit(1)
    } else {
        println!("{}", value.blue());
    }

    return;
}

// this fn takes a str as input and watches for the return or r key based on which wither it calls the commit getter again or accepts the result.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InputAction {
    Accept,
    Retry,
    Quit,
}

pub async fn msg_handler(_value: &str, _in_handler: bool) -> Result<InputAction, io::Error> {
    println!(
        "{}",
        "[enter]: accept | [r]: get a new commit message | [q]: exit".magenta()
    );
    enable_raw_mode()?;
    loop {
        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                match key.code {
                    KeyCode::Enter => {
                        disable_raw_mode()?;
                        return Ok(InputAction::Accept);
                    }
                    KeyCode::Char('r') => {
                        disable_raw_mode()?;
                        println!("{}", "Getting a new message...".green());
                        return Ok(InputAction::Retry);
                    }
                    KeyCode::Char('q') => {
                        disable_raw_mode()?;
                        return Ok(InputAction::Quit);
                    }
                    _ => {}
                }
            }
        }
    }
}
