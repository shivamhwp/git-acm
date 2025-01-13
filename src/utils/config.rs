use std::{
    env, fs,
    io::Error,
    path::{Path, PathBuf},
};

use yansi::Paint;

fn get_config_dir() -> PathBuf {
    let home = env::var("HOME").expect("couldn't get the home dir");
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

pub fn save_value(value: &str) -> std::io::Result<()> {
    config_exists()?;
    let config_file = get_config_dir().join("model.txt");
    fs::write(config_file, value)
}
