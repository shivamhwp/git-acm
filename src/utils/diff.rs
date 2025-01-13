use duct::cmd;
use yansi::Paint;

pub fn get_diff() -> String {
    let no_diff_err_message = "🤔 are the changes staged ?".red().to_string();

    match cmd!("git", "diff", "--staged", "--color=always").read() {
        Ok(r) => {
            return r;
        }
        Err(_e) => {
            println!("{}", no_diff_err_message);
            println!("{}", "💡 try `git add <file_name>` to stage changes".red());
            return "".to_string();
        }
    }
}

pub fn is_git_initialized() {
    let no_git_err_message = "🚨 not a git repo ".red().to_string();

    match cmd!("git", "rev-parse", "--is-inside-work-tree").read() {
        Ok(_r) => {
            "".to_string();
        }
        Err(_e) => {
            println!("{}", no_git_err_message);
            println!("{}", "💡 try `git init` to initialise a git repo".red());
            return;
        }
    }
}
