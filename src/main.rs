use clap::Command;

use models::{anthropic::anthropic, gemini::gemini, llama::llama, openai::openai};
use utils::{
    checks::Check,
    config::{load_value, print_to_cli, save_autocommit_preference, save_value},
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
                .subcommand(Command::new("llama"))
                .override_help("choose from openai, anthropic, gemini and llama."),
        )
        .subcommand(
            Command::new("autocommit")
                .about("enable or disable the autocommit functionality")
                .subcommand(Command::new("enable"))
                .subcommand(Command::new("disable"))
                .override_help("enable or disable the auto-commit functionality"),
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
                Some(("llama", _)) => save_value("llama"),
                _ => {
                    println!("{}", "choose an api to make requests.".red());
                    println!(
                        "{}",
                        "available options: [ openai | anthropic | gemini | llama ] ".yellow()
                    );
                    return;
                }
            }
            get_commit_msg();
        }
        Some(("autocommit", sub_matches)) => match sub_matches.subcommand() {
            Some(("enable", _)) => {
                save_autocommit_preference("enable");
            }
            Some(("disable", _)) => {
                save_autocommit_preference("disable");
            }
            _ => {
                println!(
                    "{}",
                    "invalid. available commands : enable or disable".red()
                );
                return;
            }
        },
        _ => {
            get_commit_msg();
        }
    }
}

fn get_commit_msg() {
    let model = load_value();

    match model.as_str() {
        "openai" => {
            Check::is_response_empty(&openai());
            print_to_cli(&openai());
        }
        "anthropic" => {
            Check::is_response_empty(&anthropic());
            print_to_cli(&openai());
        }
        "gemini" => {
            Check::is_response_empty(&gemini());
            print_to_cli(&gemini());
        }
        "llama" => {
            Check::is_response_empty(&llama());
            print_to_cli(&llama());
        }
        _ => {
            println!("{}", "   no default api found.".red());
            println!(
                "{}",
                "💡 choose from [ openai | anthropic | gemini | llama ].".green()
            );
            std::process::exit(1)
        }
    }
}
