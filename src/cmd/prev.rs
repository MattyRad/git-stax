use std::process::Command;

pub fn prev(available_pasts: Vec<String>) {
    match get_prev(available_pasts) {
        Some(prev) => {
            // TODO: replace with git2
            Command::new("git")
                .arg("switch")
                .arg(prev.clone())
                .output()
                .expect("Failed to execute git switch");

            println!("Switched to branch: {}", prev);
        }
        None => {
            println!("Already at most ancient");
        }
    }
}

pub fn get_prev(available_pasts: Vec<String>) -> Option<String> {
    if available_pasts.is_empty() {
        return None;
    }

    let prev = &available_pasts[0];

    Some(prev.to_string())
}
