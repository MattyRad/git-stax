use git2::{Repository, Oid};

use crate::cmd::get_prev;
use crate::StaxBranch;
use git2::Error;

use inquire::Confirm;

pub fn sync(repo: &Repository, mut cb: StaxBranch, all_branches: Vec<String>) {
    let option_prev = get_prev(cb.get_pasts(all_branches));

    match option_prev {
        Some(prev) => {

            let current_head = cb.branch.get().peel_to_commit().unwrap().id();

            // For now, we'll only compare two branches checking if the parent contains the HEAD.
            // While I think it's technically possible that two branches aren't equal while
            // still containing HEAD, I don't think that's a common case.
            match branch_contains_commit(repo, prev.as_str(), current_head) {
                Ok(does_have_head) => {
                    if !does_have_head {
                        println!("Parent branch {} doesnt have head, the stack remains; ignoring", cb.name);
                    } else {
                        repo.set_head(format!("refs/heads/{}", prev).as_str())
                            .unwrap();

                        let ans = Confirm::new(
                            format!(
                                "Parent branch {} contains HEAD, delete current branch?",
                                cb.name
                            )
                            .as_str(),
                        )
                        .with_default(false)
                        .with_help_message(
                            "Note that this is based on local info, remotes may differ",
                        )
                        .prompt();

                        match ans {
                            Ok(true) => println!("Deleted, switched to {}", prev),
                            Ok(false) => (),
                            Err(_) => println!("Error with questionnaire, try again later"),
                        }

                        cb.branch.delete().unwrap();
                    }
                }
                Err(..) => println!("Error comparing branches"),
            }
        }
        None => println!("Up to date"),
    };
}

fn branch_contains_commit(repo: &Repository, branch_name: &str, target_oid: Oid) -> Result<bool, Error> {
    let branch = repo.find_branch(branch_name, git2::BranchType::Local)?;

    let branch_oid = branch.get().peel_to_commit()?.id();

    let contains_commit = repo.graph_descendant_of(branch_oid, target_oid)?;

    Ok(contains_commit)
}
