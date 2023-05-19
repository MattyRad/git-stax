use clap::{Parser, Subcommand};
use git2::{Branch, Repository};

pub mod cmd;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Optional name to operate on
    name: Option<String>,

    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Go to the parent branch
    Prev {},
    /// Go to a/the child branch
    Next {},
    /// Creates a new branch (not recommended, use Complete instead)
    New {},
    /// When your current branch is complete, start the next stack
    Complete {
        #[arg(long, default_value_t = true)]
        create_mr: bool,
    },
    /// Pull all of the local branches, deleting any which have been merged
    Snip {
        /// Delete the current branch if it's the same as the parent
        #[arg(long, default_value_t = true)]
        delete: bool,
    },
}

pub struct StaxBranch<'a> {
    name: String,
    branch: Branch<'a>,
}

impl StaxBranch<'_> {
    pub fn get_prefix(&self) -> Option<&str> {
        let parts = self.name.split('/').collect::<Vec<_>>();

        if parts.len() > 1 {
            Some(parts[0])
        } else {
            None
        }
    }
    pub fn get_futures(&self, all_branches: Vec<String>) -> Vec<String> {
        let mut futures = Vec::new();

        for branch in all_branches {
            let mut clone = self.name.clone();
            clone.push('-');

            if branch.contains(&clone) {
                futures.push(branch);
            }
        }

        futures
    }

    pub fn get_pasts(&self, all_branches: Vec<String>) -> Vec<String> {
        if !self.name.contains('-') {
            return Vec::new();
        }

        let mut pasts = Vec::new();
        let trimmed = self.name.split('-').next().unwrap();

        for branch in all_branches {
            if branch.contains(trimmed) && branch.len() < self.name.len() {
                pasts.push(branch.to_string());
            }
        }

        // FIXME: 10000; better sort
        pasts.sort_by_key(|past| 10000 - past.len());

        pasts
    }
}

fn main() {
    let cli: Cli = Cli::parse();

    let repo = match Repository::open(".") {
        Ok(repo) => repo,
        Err(e) => panic!("failed to open: {}", e),
    };

    let branches = match repo.branches(Some(git2::BranchType::Local)) {
        Ok(branches) => branches,
        Err(_) => panic!("Couldn't get branches"),
    };

    let head = repo.head().unwrap();

    if !head.is_branch() {
        panic!("HEAD is not a branch");
    }

    let branch_name = head.shorthand().unwrap();

    let current_branch_name = branch_name.to_string();

    let branch = repo.find_branch(current_branch_name.as_str(), git2::BranchType::Local);

    let cb = StaxBranch {
        name: current_branch_name,
        branch: branch.unwrap(),
    };

    let mut branch_names = vec![];

    for branch_result in branches {
        let branch_tuple = branch_result.unwrap();

        let branch = branch_tuple.0;

        let name = branch.name().unwrap().unwrap().to_string();

        branch_names.push(name);
    }

    match &cli.command {
        Some(Commands::Prev {}) => cmd::prev(cb.get_pasts(branch_names)),
        Some(Commands::Next {}) => cmd::next(cb.get_futures(branch_names)),
        Some(Commands::New {}) => cmd::new(cb),
        Some(Commands::Complete { .. }) => cmd::complete(&repo, cb, branch_names),
        Some(Commands::Snip { .. }) => cmd::sync(&repo, cb, branch_names),
        None => (),
    };
}
