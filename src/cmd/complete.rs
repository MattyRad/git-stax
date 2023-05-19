use std::process::Command;

use crate::StaxBranch;
use git2::Repository;

use crate::cmd::new;

pub fn get_mr_title(msg: String) -> String {
    let parts = msg.split(':').collect::<Vec<_>>();

    if parts.len() > 1 {
        parts[1].to_string()
    } else {
        parts[0].to_string()
    }
}

pub fn complete(repo: &Repository, cb: StaxBranch, all_branches: Vec<String>) {
    let binding = cb.get_pasts(all_branches);
    let previous_branch_name = binding.first().unwrap();

    let head = repo.head().unwrap();
    let binding2 = head.peel_to_commit().unwrap();
    let msg = binding2.message().unwrap();
    let mr_title = get_mr_title(msg.to_string());

    // TODO: replace with git2
    Command::new("git")
        .arg("push")
        .arg("--push-option merge_request.create")
        .arg("--push-option merge_request.label=\"stacked\"")
        .arg("--push-option merge_request.target=".to_owned() + previous_branch_name)
        .arg("--push-option merge_request.title=".to_owned() + mr_title.as_str())
        .output()
        .expect("Failed to execute git push");

    new(cb);
}
