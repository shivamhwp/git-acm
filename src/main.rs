use clap::{Command, Arg};
use serde::Deserialize;
use models::{
    anthropic::anthropic, deepseek::deepseek, gemini::gemini, llama::llama, openai::openai,
};
use utils::{
    config::{load_model_from_pref, load_models_from_json, print_to_cli, save_autocommit_preference, save_model_value},
    diff::is_git_initialized,
};
use yansi::Paint;

mod models;
mod utils;


#[derive(Deserialize, Debug)]
pub struct Model {
    model_api_name: String,
    model_name: String,
    provider: String,
}


fn main() {
    is_git_initialized();
    let description = " instant meaningful commit messages.\n (more): https://git.new/git-acm "
        .magenta().bold()
        .to_string();

    let cli = Command::new("git-acm")
        .author("shivam [shivam.ing]")
        .version("1.3.0") 
        .about(description)
        .subcommand(
            Command::new("use")
            .about("choose which model to use, run git-acm list to see the available models")
            .arg(Arg::new("model").required(true))
        )

        .subcommand(
            Command::new("autocommit")
                .about("enable or disable autocommit functionality")
                .subcommand(Command::new("enable"))
                .subcommand(Command::new("disable"))
        )
        .subcommand(
            Command::new("list")
                .about("lists all supported models")
        )
        .get_matches();


    match cli.subcommand() {
        Some(("run", _)) => {
            get_commit_msg();
        }

        Some(("list", _)) => {
            let models = load_models_from_json();
            println!("{}", " available models".green());
            for model in models {
                println!(" {}", model.model_name);
            }
        }
        Some(("use", sub_matches)) => {
            let user_model_input = sub_matches.get_one::<String>("model").unwrap().to_string();
            let models = load_models_from_json();
            match models.iter().find(|m| m.model_name == user_model_input)// using model_name here
             {
                Some(model) => {
                    save_model_value(&model.model_api_name); // save the model api name to the config file
                    get_commit_msg();
                }
                None => {
                    println!(
                        "{}",
                        "run git-acm list to see the available models".yellow(),
                    );
                    std::process::exit(1);
                }
            }
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
                    "invalid. available options : enable or disable".red()
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
    let model = load_model_from_pref(None);   
    let models_from_file = load_models_from_json();
    match models_from_file.iter().find(|m| m.model_api_name == model) {
        Some(model) => {
            println!("using model: {}", model.model_name.magenta());
            match model.provider.as_str() {
                "openai" => {
                    print_to_cli(&openai());
                }
                "anthropic" => {
                    print_to_cli(&anthropic());
                }
                "gemini" => {
                    print_to_cli(&gemini());
                }
                "llama" => {
                    print_to_cli(&llama());
                }
                "deepseek" => {
                    print_to_cli(&deepseek());
                }
                _ => {
                    println!(
                        "{}{}",
                        "💡 choose from ".green(),
                        models_from_file.iter().map(|m| m.model_api_name.clone()).collect::<Vec<String>>().join(" | ")
                    );
                    std::process::exit(1);
                }
            }
        }
        None => {
            println!("{}", "model not supported".red());
            std::process::exit(1);
        }
    }
}
