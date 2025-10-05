use arboard::Clipboard;
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode},
};
use duct::cmd;
use std::{
    env, fs,
    io::{self, BufReader, BufWriter},
    path::{PathBuf},
};
use yansi::Paint;

use serde::{Deserialize, Serialize};

// Centralized default model
fn default_model() -> StoredModel {
    StoredModel {
        name: "Google: Gemini 2.5 Flash Preview 09-2025".to_string(),
        canonical_slug: "google/gemini-2.5-flash-preview-09-2025".to_string(),
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Preferences {
    pub default_model: StoredModel,
    pub user_selected_model: StoredModel,
    pub auto_commit: bool,
}

fn get_config_dir() -> PathBuf {
    directories::ProjectDirs::from("", "", "git-acm")
        .expect("Configuration directory not available on this platform")
        .config_dir()
        .to_path_buf()
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
        let default_model = default_model();
        let default_prefs = Preferences {
            default_model: default_model.clone(),
            user_selected_model: default_model,
            auto_commit: false,
        };
        return Ok(default_prefs);
    }
    let config_file = config_file_path();
    if !config_file.exists() {
        let default_model = default_model();
        let default_prefs = Preferences {
            default_model: default_model.clone(),
            user_selected_model: default_model,
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

pub fn load_model_from_pref() -> StoredModel {
    match load_preferences() {
        Ok(prefs) => prefs.user_selected_model,
        Err(_) => default_model(),
    }
}

pub fn load_auto_commit() -> bool {
    match load_preferences() {
        Ok(prefs) => prefs.auto_commit,
        Err(_) => false,
    }
}

fn try_update_preferences<F>(update: F) -> Result<(), Box<dyn std::error::Error>>
where
    F: FnOnce(&mut Preferences),
{
    let mut prefs = load_preferences()?;
    update(&mut prefs);
    save_preferences(&prefs)?;
    Ok(())
}

pub fn save_model_value(model: &StoredModel) {
    match try_update_preferences(|prefs| prefs.user_selected_model = model.clone()) {
        Ok(()) => println!("{} {}", model.canonical_slug, " saved as default.".green()),
        Err(e) => println!(
            "{} {}",
            model.canonical_slug,
            format!(" couldn't save: {}", e).red()
        ),
    }
}

pub fn save_auto_commit(enabled: bool) {
    let status = if enabled { "enabled" } else { "disabled" };
    match try_update_preferences(|prefs| prefs.auto_commit = enabled) {
        Ok(()) => println!("autocommit {}", format!("{}", status).green()),
        Err(e) => println!(
            "autocommit {} failed to save: {}",
            status,
            format!("{}", e).red()
        ),
    }
}

pub fn get_api_key() -> String {
    match env::var("OPENROUTER_API_KEY") {
        Ok(k) => k,
        Err(_) => String::new(),
    }
}

pub fn copy_to_clipboard(text: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut clipboard = Clipboard::new()?;
    clipboard.set_text(text)?;
    println!("{}", "copied to clipboard 👍".magenta());
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
        println!("{}", format!("failed to create config dir: {}", e).red());
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

struct RawModeGuard;

impl Drop for RawModeGuard {
    fn drop(&mut self) {
        let _ = disable_raw_mode();
    }
}

pub async fn msg_handler() -> Result<InputAction, io::Error> {
    println!(
        "{}",
        "[enter]: accept | [r]: get a new commit message | [q]: exit".magenta()
    );
    enable_raw_mode()?;
    let _guard = RawModeGuard;
    loop {
        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                match key.code {
                    KeyCode::Enter => {
                        return Ok(InputAction::Accept);
                    }
                    KeyCode::Char('r') => {
                        return Ok(InputAction::Retry);
                    }
                    KeyCode::Char('q') => {
                        return Ok(InputAction::Quit);
                    }
                    _ => {}
                }
            }
        }
    }
}
