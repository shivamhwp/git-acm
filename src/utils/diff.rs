use duct::cmd;
use yansi::Paint;

use crate::utils::checks::Check;

fn default_excludes() -> Vec<&'static str> {
    vec![
        ":(exclude)node_modules/**",
        ":(exclude)dist/**",
        ":(exclude)build/**",
        ":(exclude)target/**",
        ":(exclude)vendor/**",
        ":(exclude).next/**",
        ":(exclude)out/**",
        ":(exclude)*.min.js",
        ":(exclude)*.min.mjs",
        ":(exclude)*.min.css",
        ":(exclude)*.map",
        ":(exclude)package-lock.json",
        ":(exclude)bun.lockb",
        ":(exclude)bun.lock",
        ":(exclude)pnpm-lock.yaml",
        ":(exclude)yarn.lock",
        ":(exclude)Cargo.lock",
    ]
}

fn build_git_diff_args() -> Vec<String> {
    let mut args: Vec<String> = vec![
        "diff".into(),
        "--staged".into(),
        "--unified=0".into(),
        "--no-color".into(),
        "--".into(),
        ".".into(),
    ];
    for p in default_excludes() {
        args.push(p.to_string());
    }
    args
}

pub fn get_diff() -> String {
    let args = build_git_diff_args();
    match cmd("git", args).read() {
        Ok(result) => {
            Check::is_diff_empty(&result);
            return result;
        }
        Err(_) => {
            println!("{}", "failed to read staged diff".red());
            println!("{}", "ensure Git is installed and run inside a Git repo".red());
            std::process::exit(1)
        }
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
