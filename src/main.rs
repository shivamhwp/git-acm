use clap::Command;
use models::{anthropic::anthropic, gemini::gemini, openai::openai};
use std::fs;
use yansi::Paint;

mod models;
mod utils;

fn main() {
    let description = "generate meaningful commit messages locally using AI. go to https://github.com/shivamhwp/git-acm for more details."
        .blue()
        .to_string();
    // let run_command = "explicit run command, does the same thing as running `git-acm`";

    let cli = Command::new("git-acm")
        .author("shivam [shivam.ing]")
        .version("0.1.2") // similar to cargo.toml file.
        .about(description)
        .subcommand(
            Command::new("use")
                .about("choose the api's you want to use ")
                .subcommand(Command::new("openai"))
                .subcommand(Command::new("anthropic"))
                .subcommand(Command::new("gemini"))
                .override_help("choose from openai, anthropic or gemini"),
        )
        .get_matches();

    match cli.subcommand() {
        Some(("run", _)) => {
            get_commit_msg();
        }
        Some(("use", sub_matches)) => {
            match sub_matches.subcommand() {
                Some(("openai", _)) => {
                    save_value("openai").expect("the default api was not set to openai")
                }
                Some(("anthropic", _)) => {
                    save_value("anthropic").expect("the default api was not set to anthropic")
                }
                Some(("gemini", _)) => {
                    save_value("gemini").expect("the default value was not set to gemini")
                }
                _ => {
                    println!("{}", "choose an api to make requests".red())
                }
            }
            get_commit_msg();
        }
        _ => {
            get_commit_msg();
        }
    }
}

fn get_commit_msg() {
    let model = load_value();
    match model.as_str() {
        "openai" => openai(),
        "anthropic" => anthropic(),
        "gemini" => gemini(),
        _ => {
            println!("{}", "no default api found".red());
            println!(
                "{}",
                "no api selected. choose from  [ openai | anthropic | gemini ]".red()
            )
        }
    }
}

fn save_value(value: &str) -> std::io::Result<()> {
    return fs::write("assets/model.txt", value);
}

fn load_value() -> String {
    let model = "assets/model.txt";

    match fs::read_to_string(model) {
        Ok(s) => {
            return s;
        }
        Err(e) => {
            println!(
                "{}",
                "couldn't get the default model, running with gemini once".red()
            );
            println!("{}", e);
            return "gemini".to_string();
        }
    }
}
