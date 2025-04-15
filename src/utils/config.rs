use arboard::Clipboard;
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode},
};
use duct::cmd;
use std::{
    env, fs,
    io::Error,
    path::{Path, PathBuf},
};

use yansi::Paint;

use crate::{get_commit_msg, Model};


use serde::Deserialize;
#[derive(Deserialize, Debug)]
struct Models {
    models: Vec<Model>,
}



fn get_config_dir() -> PathBuf {
    let home = env::var("HOME").unwrap_or_else(|_| ".".to_string());
    return Path::new(&home).join(".config").join("git-acm");
}

fn config_exists() -> Result<(), Error> {
    let config_dir = get_config_dir();
    if !config_dir.exists() {
        return fs::create_dir(&config_dir);
    }
    Ok(())
}

pub fn load_model_from_pref(provider: Option<&str>) -> String {
    let config_file = get_config_dir().join("model.txt");

    if let Err(e) = config_exists() {
        println!(
            "{}",
            format!(
                "Failed to create config dir {}",
                e
            )
            .red()
        );

        
        match provider {
            Some(provider) => {
                match provider {
                    "anthropic" => {
                        return "claude-3.5-sonnet".to_string();
                    }
                    "gemini" => {
                        return "gemini-2.0-flash".to_string();
                    }
                    "deepseek" => {
                        return "deepseek-chat".to_string();
                    }
                    "llama" => {
                        return "llama-3.2-3b-instruct".to_string();
                    }
                    "openai" => {
                        return "gpt-4.1".to_string();
                    }
                    _ => {
                        return "gemini-2.0-flash".to_string();
                    }
                }
            }
            None => {
                return "gemini-2.0-flash".to_string();
            }
        }
    }

    if !config_file.exists() {
        if let Err(e) = fs::write(&config_file, "gemini-2.0-flash") {
            println!(
                "{}",
                format!(
                    "Failed to create config file, setting gemini 2.5 pro as default {}",
                    e
                )
                .red()
            );
            return "gemini-2.0-flash".to_string();
        }
    }

    match fs::read_to_string(config_file) {
        Ok(s) => s.trim().to_string(),
        Err(e) => {
            println!(
                "{}",
                format!("Error reading config, using gemini-2.0-flash as default {}", e).red()
            );
            "gemini-2.0-flash".to_string()
        }
    }   
}

pub fn load_auto_commit_value() -> String {
    let auto_commit = get_config_dir().join("autocommit.txt");

    if !auto_commit.exists() {
        if let Err(e) = fs::write(&auto_commit, "disable") {
            println!("{}", format!("error with autocommit.txt file {}", e).red());
            return String::from("disable");
        }
    }

    match fs::read_to_string(auto_commit) {
        Ok(a) => return a.trim().to_string(),
        Err(e) => {
            println!(
                "{}",
                format!("Error reading autocommit, set as disable {}", e).red()
            );
            return String::from("disable");
        }
    }
}


pub fn save_model_value(value: &str) {
    if config_exists().is_err() {
        println!("{}", "config doesn't exist ".red());
        return;
    };
    let config_file = get_config_dir().join("model.txt");

    match fs::write(config_file, value) {
        Ok(_ok) => {
            println!("{}{}", value, " saved as default.".green())
        }
        Err(_e) => {
            println!("{}{}", value, "i couldn't save it, as a default. 😔".red())
        }
    }
}

pub fn save_autocommit_preference(value: &str) {
    let auto_commit = get_config_dir().join("autocommit.txt");
    if config_exists().is_err() {
        println!("{}", "config doesn't exist ".red());
        return;
    };

    match fs::write(auto_commit, value) {
        Ok(_ok) => {
            println!("{}{}d", "autocommit ".green(), value)
        }
        Err(_e) => {
            println!("{}{}", value, "i couldn't save it, as a default. 😔".red())
        }
    }
}

pub fn get_api_key(value: &str) -> String {
    let key = format!("{}_API_KEY", value.to_uppercase());

    match env::var(key) {
        Ok(k) => {
            return String::from(k);
        }
        Err(_e) => {
            println!("{}", "couldn't get the api key".red());
            return String::from("disable");
        }
    }
}

pub fn get_api_url(value: &str, default: &str) -> String {
    let key = format!("{}_API_URL", value.to_uppercase());
    match env::var(key) {
        Ok(k) => {
            return String::from(k);
        }
        Err(_e) => {
            println!("{}", "couldn't get the api url ".red());
            return String::from(default);
        }
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

pub fn run_git_commit(value: &str) {
    let preference = load_auto_commit_value();
    match preference.as_str() {
        "enable" => {
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
        }
        "disable" => {
            // println!("{}", "autocommit is disabled".yellow());
            // println!("{}", "run `git-acm autocommit enable`".magenta());
            return;
        }
        _ => {
            println!("{}", "invalid autocommit value.".red());
            println!("{}", "cd ~/.config/git-acm. open the autocommit.txt file. and either write `enable` or `disable`.");
            return;
        }
    }
}

pub fn print_to_cli(value: &str) {
    if value.is_empty() {
        println!("{}", "got no response".red());
        std::process::exit(1)
    } else {
        println!("{}", value.blue());
        match msg_handler(value, false) {
            Ok(_v) => {}
            _ => {
                println!("{}", "invalid input".red());
                std::process::exit(1);
            }
        }
    }

    return;
}

// this fn takes a str as input and watches for the return or r key based on which wither it calls the commit getter again or accepts the result.
pub fn msg_handler(value: &str, in_handler: bool) -> Result<(), Error> {
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
                        println!("{}", value);
                        // the follwing fns are here. so that these only run once.
                        copy_to_clipboard(value).unwrap_or_else(|_x| {
                            println!("{}", "error copying the result to clipboard".yellow())
                        });
                        run_git_commit(value);
                        return Ok(());
                    }
                    KeyCode::Char('r') => {
                        disable_raw_mode()?;
                        println!("{}", "getting a new message...".green());
                        if !in_handler {
                            //  to prevent the infinite loop
                            get_commit_msg();
                        }
                        return Ok(());
                    }
                    KeyCode::Char('q') => {
                        disable_raw_mode()?;
                        println!("{}", "exiting...".green());
                        std::process::exit(0);
                    }
                    _ => {
                        disable_raw_mode()?;
                        println!("{}", "invalid input".red());
                        std::process::exit(1);
                    }
                }
            }
        }
        disable_raw_mode()?;
    }
}


pub fn load_models_from_json() -> Vec<Model> {
  let models_path = include_str!("../../assets/models.json");
    match serde_json::from_str::<Models>(&models_path) {
    Ok(model_obj) => model_obj.models,
    Err(_e) => {
        println!("{}", "couldn't load models".red());
        return vec![];
    }
}
}
