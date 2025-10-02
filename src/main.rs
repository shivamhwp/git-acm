use clap::{Arg, Command};
use yansi::Paint;

mod utils;

use utils::{
    config::{
        load_model_from_pref, load_models_list, print_to_cli, save_auto_commit, save_model_value,
    },
    diff::is_git_initialized,
    openrouter::{fetch_and_store_models, get_commit_message_from_openrouter},
};

const VERSION: &str = "1.3.0";
const AUTHOR: &str = "shivam [shivam.ing]";

#[tokio::main]
async fn main() {
    is_git_initialized();

    let cli = build_cli().get_matches();

    match cli.subcommand() {
        Some(("list", _)) => list_models(),
        Some(("get-models", _)) => {
            if let Err(e) = fetch_and_store_models().await {
                eprintln!("{}", format!("Failed to get models: {}", e).red());
                std::process::exit(1);
            }
        }
        Some(("use", sub_matches)) => handle_model_selection(sub_matches).await,
        Some(("autocommit", sub_matches)) => handle_autocommit(sub_matches),
        Some(("run", _)) | None => generate_commit_message().await,
        _ => unreachable!("Unhandled subcommand"),
    }
}

fn build_cli() -> Command {
    let description = " instant meaningful commit messages.\n (more): https://git.new/git-acm "
        .magenta()
        .bold()
        .to_string();

    Command::new("git-acm")
        .author(AUTHOR)
        .version(VERSION)
        .about(description)
        .subcommand(Command::new("list").about("Lists all supported models"))
        .subcommand(Command::new("get-models").about("Fetch and store models from OpenRouter"))
        .subcommand(
            Command::new("use")
                .about("Choose which model to use (run 'git-acm list' to see available models)")
                .arg(Arg::new("model").required(true).help("Model name to use")),
        )
        .subcommand(
            Command::new("autocommit")
                .about("Enable or disable autocommit functionality")
                .subcommand(Command::new("enable").about("Enable autocommit"))
                .subcommand(Command::new("disable").about("Disable autocommit")),
        )
        .subcommand(Command::new("run").about("Generate a commit message (default behavior)"))
}

fn list_models() {
    println!("{}", " available models".green());
    // Prefer stored models if present
    match load_models_list() {
        Ok(models) => {
            for m in models {
                println!("  • {}", m.name);
            }
        }
        Err(e) => {
            eprintln!("{}", format!("Failed to load models: {}", e).red());
        }
    }
}

async fn handle_model_selection(sub_matches: &clap::ArgMatches) {
    let model_name = sub_matches
        .get_one::<String>("model")
        .expect("Model argument is required");

    // Try stored models first
    if let Ok(models) = utils::config::load_models_list() {
        if let Some(found) = models.iter().find(|m| m.canonical_slug == *model_name) {
            save_model_value(&found.canonical_slug);
            generate_commit_message().await;
            return;
        }
    }
    eprintln!("{}", format!("Model '{}' not found", model_name).red());
    eprintln!(
        "{}",
        "Run 'git-acm get-models' and then 'git-acm list'".yellow()
    );
    std::process::exit(1);
}

fn handle_autocommit(sub_matches: &clap::ArgMatches) {
    match sub_matches.subcommand() {
        Some(("enable", _)) => save_auto_commit(true),
        Some(("disable", _)) => save_auto_commit(false),
        _ => {
            eprintln!("{}", "Invalid option. Use 'enable' or 'disable'".red());
            eprintln!("{}", "Example: git-acm autocommit enable".yellow());
            std::process::exit(1);
        }
    }
}

pub async fn generate_commit_message() {
    let chosen_model = load_model_from_pref(None);

    println!("{} {}", "Using model:".cyan(), chosen_model.magenta());

    // Pass the canonical slug (provider/model) to OpenRouter
    let commit_message = get_commit_message_from_openrouter(&chosen_model).await;
    print_to_cli(&commit_message);
}
