use clap::Command;

mod models;
mod utils;
use models::{anthropic::anthropic, gemini::gemini, openai::openai};
use yansi::Paint;

pub enum Models {
    Openai,
    Anthropic,
    Gemini,
}

fn main() {
    let description = "generate meaningful commit messages locally using AI. go to https://github.com/shivamhwp/git-acm for more details."
        .blue()
        .to_string();
    let run_command = "explicit run command, does the same thing as running `git-acm`";

    let cli = Command::new("git-acm")
        .author("shivam [shivam.ing]")
        .version("0.1.2") // similar to cargo.toml file.
        .about(description)
        .subcommand(Command::new("run").about(run_command))
        .subcommand(
            Command::new("use")
                .override_help("choose from openai, anthropic or gemini")
                .about("choose the api's you want to use ")
                .subcommand(Command::new("openai"))
                .about("runs openai api")
                .subcommand(Command::new("anthropic"))
                .about("runs anthropic api")
                .subcommand(Command::new("gemini"))
                .about("choose from openai, anthropic or gemini"),
        )
        .get_matches();

    match cli.subcommand() {
        Some(("run", _)) => {
            get_commit_msg();
        }
        Some(("use", sub_matches)) => match sub_matches.subcommand() {
            Some(("openai", _)) => {
                println!("openai");
                openai();
            }
            Some(("anthropic", _)) => {
                println!("anthropic");
                anthropic();
            }
            Some(("gemini", _)) => {
                println!("gemini");
                gemini();
            }
            _ => println!(
                "No valid subcommand provided, available commands : openai , anthropic & gemini"
            ),
        },
        None => {
            get_commit_msg();
        }
        _ => {
            get_commit_msg();
        }
    }
}

fn get_commit_msg() {
    gemini();
}
