use std::process::Command;

use crate::StaxBranch;

use rand::{distributions::Alphanumeric, Rng};

fn random_str(len: usize) -> String {
    let s: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(len)
        .map(char::from)
        .collect();

    s
}

pub fn new(cb: StaxBranch) {
    let new_branch_name = cb.name + "-" + &random_str(3);

    // TODO: replace with git2
    Command::new("git")
        .arg("switch")
        .arg("--create")
        .arg(new_branch_name)
        .output()
        .expect("Failed to execute git push");
}
