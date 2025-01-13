use arboard::Clipboard;
use std::{
    env, fs,
    io::Error,
    path::{Path, PathBuf},
};

use yansi::Paint;

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

pub fn load_value() -> String {
    let config_file = get_config_dir().join("model.txt");

    if let Err(e) = config_exists() {
        println!(
            "{}",
            format!(
                "Failed to create config dir, setting openai as default: {}",
                e
            )
            .red()
        );
        return "openai".to_string();
    }

    if !config_file.exists() {
        if let Err(e) = fs::write(&config_file, "openai") {
            println!(
                "{}",
                format!(
                    "Failed to create config file, setting openai as default {}",
                    e
                )
                .red()
            );
            return "openai".to_string();
        }
    }
    match fs::read_to_string(config_file) {
        Ok(s) => s.trim().to_string(),
        Err(e) => {
            println!(
                "{}",
                format!("Error reading config, using openai as default {}", e).red()
            );
            "openai".to_string()
        }
    }
}

pub fn save_value(value: &str) {
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

pub fn get_api_key(value: &str) -> String {
    let key = format!("{}_API_KEY", value.to_uppercase());

    match env::var(key) {
        Ok(k) => {
            return k.to_string();
        }
        Err(_e) => {
            println!("{}", "couldn't get the api key".red());
            println!(
                "{}",
                "either export the key in terminal or define them in .env"
            );
            return "".to_string();
        }
    }
}

pub fn get_api_url(value: &str, default: &str) -> String {
    let key = format!("{}_API_URL", value.to_uppercase());
    match env::var(key) {
        Ok(k) => {
            return k.to_string();
        }
        Err(_e) => {
            println!("{}", "couldn't get the api url ".red());
            return default.to_string();
        }
    }
}

pub fn copy_to_clipboard(text: &str) -> Result<(), Box<dyn std::error::Error>> {
    match Clipboard::new()?.set_text(text) {
        Ok(_t) => {
            println!("{}", "( automatically copied to clipboard 👍)".magenta());
        }
        Err(_e) => {
            println!("{}", "( couldn't copy to clipboard 🥲 )".yellow());
        }
    }
    Ok(())
}
