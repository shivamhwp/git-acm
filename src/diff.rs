use duct::cmd;

pub fn get_diff() {
    let diff = cmd!("git", "diff", "--color=always").read().unwrap();
    println!("{}", &diff);
}
