use duct::cmd;
use yansi::Paint;

pub fn get_diff() -> String {
    is_git_initialized();
    let no_diff_err_message = "are the changes staged ?".red().to_string();

    let diff = cmd!("git", "diff", "--staged", "--color=always")
        .read()
        .expect(&no_diff_err_message);
    // println!("here is the diff \n{}\n", &diff);
    return diff;
}

pub fn is_git_initialized() {
    let no_git_err_message = "not a git repo".red().to_string();

    let git_init_check = cmd!("git", "rev-parse", "--is-inside-work-tree")
        .read()
        .expect(&no_git_err_message);

    if git_init_check != "true" {
        println!("{}", "initialize git to use git-acm".red());
        return;
    }
}
