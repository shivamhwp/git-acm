use clap::Command;
use models::{anthropic::anthropic, gemini::gemini, openai::openai};
use yansi::Paint;

mod models;
mod utils;

struct Model {
    model: String,
}

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
            println!("{:?}", sub_matches);
            get_commit_msg();
        }
        _ => {
            get_commit_msg();
        }
    }
}

fn get_commit_msg() {
    openai();
    // anthropic();
    // gemini();
}
