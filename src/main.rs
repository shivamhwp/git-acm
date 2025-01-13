use clap::Command;
use models::{anthropic::anthropic, gemini::gemini, openai::openai};
use utils::{
    config::{load_value, save_value},
    diff::is_git_initialized,
};
use yansi::Paint;

mod models;
mod utils;

fn main() {
    is_git_initialized();
    let description = "
  generate meaningful commit messages locally using AI. go to https://github.com/shivamhwp/git-acm for more details."
        .magenta().bold()
        .to_string();
    // let run_command = "explicit run command, does the same thing as running `git-acm`";

    let cli = Command::new("git-acm")
        .author("shivam [shivam.ing]")
        .version("1.0.2") // similar to cargo.toml file.
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
                Some(("openai", _)) => save_value("openai"),
                Some(("anthropic", _)) => save_value("anthropic"),
                Some(("gemini", _)) => save_value("gemini"),
                _ => {
                    println!("{}", "choose an api to make requests".red());
                    return;
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
            println!("{}", "   no default api found.".red());
            println!(
                "{}",
                "💡 choose from [ openai | anthropic | gemini ].".green()
            );
            return;
        }
    }
}
