use inquire::{error::InquireError, Select};
use std::process::Command;

pub fn next(available_futures: Vec<String>) {
    if available_futures.len() == 1 {
        // TODO: replace with git2
        Command::new("git")
            .arg("switch")
            .arg(available_futures.first().unwrap())
            .output()
            .expect("Failed to execute git switch");
    } else if available_futures.len() > 1 {
        let asstr = available_futures.iter().map(|s| s.as_str()).collect();

        let ans: Result<&str, InquireError> =
            Select::new("Multiple futures detected:", asstr).prompt();

        match ans {
            Ok(choice) => {
                // TODO: replace with git2
                Command::new("git")
                    .arg("switch")
                    .arg(choice)
                    .output()
                    .expect("Failed to execute git switch");

                return;
            }
            Err(_) => println!("There was an error, please try again"),
        }
    } else {
        println!("No futures available");
    }
}
