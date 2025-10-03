use clap::{Arg, Command};
use yansi::Paint;

mod utils;

use utils::{
    config::{
        copy_to_clipboard, load_model_from_pref, load_models_list, msg_handler, print_to_cli,
        run_git_commit, save_auto_commit, save_model_value, InputAction,
    },
    diff::is_git_initialized,
    openrouter::{fetch_and_store_models, get_commit_message_from_openrouter},
};

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
        .version(env!("CARGO_PKG_VERSION"))
        .about(description)
        .subcommand(Command::new("list").about("Lists all supported models"))
        .subcommand(Command::new("get-models").about("fetch models from openrouter and store them (don't run it too much (you might get rate limited)"))
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
    println!("{}", "Available models".green());
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
        let target = model_name.to_lowercase();

        if let Some(found) = models.iter().find(|m| m.canonical_slug == *model_name) {
            save_model_value(found);
            generate_commit_message().await;
            return;
        }

        if let Some(found) = models
            .iter()
            .find(|m| m.canonical_slug.to_lowercase() == target || m.name.to_lowercase() == target)
        {
            save_model_value(found);
            generate_commit_message().await;
            return;
        }

        let mut possible_matches: Vec<&utils::config::StoredModel> = models
            .iter()
            .filter(|m| m.canonical_slug.to_lowercase().contains(&target))
            .collect();

        if possible_matches.len() == 1 {
            let found = possible_matches.remove(0);
            save_model_value(found);
            generate_commit_message().await;
            return;
        } else if !possible_matches.is_empty() {
            eprintln!("{}", "Model not found. Did you mean:".yellow());
            for m in possible_matches.into_iter().take(5) {
                eprintln!("  - {} ({})", m.name, m.canonical_slug);
            }
            std::process::exit(1);
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

    println!(
        "{} {}",
        "Using model:".cyan(),
        chosen_model.name.magenta()
    );

    loop {
        // Pass the canonical slug (provider/model) to OpenRouter
        let commit_message =
            get_commit_message_from_openrouter(&chosen_model.canonical_slug).await;
        print_to_cli(&commit_message);

        match msg_handler(&commit_message, false).await {
            Ok(InputAction::Accept) => {
                println!("{}", &commit_message);
                copy_to_clipboard(&commit_message).unwrap_or_else(|_| {
                    println!("{}", "error copying the result to clipboard".yellow())
                });
                run_git_commit(&commit_message);
                break;
            }
            Ok(InputAction::Retry) => {
                println!("{}", "getting a new message...".green());
                continue;
            }
            Ok(InputAction::Quit) => {
                println!("{}", "exiting...".green());
                std::process::exit(0);
            }
            Err(_) => {
                println!("{}", "invalid input".red());
                std::process::exit(1);
            }
        }
    }
}
