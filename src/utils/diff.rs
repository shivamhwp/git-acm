use duct::cmd;

pub fn get_diff() -> String {
    let diff = cmd!("git", "diff", "--staged", "--color=always")
        .read()
        .unwrap();
    // println!("here is the diff \n{}\n", &diff);
    return diff;
}
