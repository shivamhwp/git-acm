use duct::cmd;
use yansi::Paint;

pub fn get_diff() -> String {
    let no_diff_err_message = " 🤔 are the changes staged ?".red().to_string();

    match cmd!("git", "diff", "--staged", "--color=always").read() {
        Ok(result) => {
            if result.is_empty() {
                println!("{}", no_diff_err_message);
                println!(
                    "{}",
                    " 💡 try `git add <file_name>` to stage changes.".red()
                );
                return String::new();
            }
            return result;
        }
        Err(_) => return String::new(),
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
