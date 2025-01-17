use duct::cmd;
use yansi::Paint;

use crate::utils::checks::Check;

pub fn get_diff() -> String {
    match cmd!("git", "diff", "--staged", "--color=always").read() {
        Ok(result) => {
            Check::is_diff_empty(&result);
            return result;
        }
        Err(_) => return "".to_string(),
    }
}

pub fn is_git_initialized() {
    let no_git_err_message = "🚨 not a git repo ".red().to_string();

    match cmd!("git", "rev-parse", "--is-inside-work-tree").read() {
        Ok(result) => {
            if result.trim() != "true" {
                println!("{}", no_git_err_message);
                println!("{}", "💡 try `git init` to initialise a git repo".red());
                std::process::exit(1);
            }
        }
        Err(_) => {
            println!("{}", no_git_err_message);
            println!("{}", "💡 try `git init` to initialise a git repo".red());
            std::process::exit(1);
        }
    }
}
