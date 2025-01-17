use yansi::Paint;
pub struct Check {}

impl Check {
    pub fn api_key_present(value: &str) {
        if value.is_empty() {
            println!(
                "{}",
                "either export the key in terminal or define them in .env"
            );
            return;
        }
    }

    pub fn api_url_present(value: &str) {
        if value.is_empty() {
            println!(
                "{}",
                "either export the key in terminal or define them in .env"
            );
            return;
        }
    }

    pub fn is_prompt_empty(value: &str) {
        if value.is_empty() {
            println!("{}", "no prompt found".red());
            return;
        }
    }

    pub fn is_diff_empty(value: &str) {
        if value.is_empty() {
            println!("{}", "🤔 are the stages changed ?".red());
            println!("{}", "💡 try `git add <file_name>`".red());
            std::process::exit(1);
        }
    }
}
