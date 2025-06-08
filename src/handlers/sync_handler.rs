
use crate::AppState;

pub fn handle_sync(_app_state: AppState, _value: Option<String>) {
    if which::which("git").is_err() {
        eprintln!("Git is not installed. Please install Git to use the sync feature.");
        return;
    }

    let notes_home = &_app_state.config.notes_home;

    let output = std::process::Command::new("git")
        .arg("rev-parse")
        .arg("--is-inside-work-tree")
        .current_dir(notes_home)
        .output();

    let not_a_repo = match &output {
        Ok(out) => !out.status.success(),
        Err(_) => true,
    };

    if not_a_repo {
        if std::process::Command::new("git")
            .arg("init")
            .current_dir(notes_home)
            .status()
            .is_err()
        {
            eprintln!("Failed to initialize a new git repository in the notes directory.");
            return;
        }
        println!("Initialized a new git repository in the notes directory.");
    }

    let status = std::process::Command::new("git")
        .arg("add")
        .arg(".")
        .current_dir(notes_home)
        .status();

    if let Err(e) = status {
        eprintln!("Failed to add files to git: {}", e);
        return;
    }
    
    if status.unwrap().success() {
        println!("Added files to git.");
    } else {
        eprintln!("No changes to commit.");
        return;
    }
    
    let commit_status = std::process::Command::new("git")
        .arg("commit")
        .arg("-m")
        .arg("Sync notes")
        .current_dir(notes_home)
        .status();
    if let Err(e) = commit_status {
        eprintln!("Failed to commit changes: {}", e);
        return;
    }
    
    if commit_status.unwrap().success() {
        println!("Committed changes to git.");
    } else {
        eprintln!("No changes to commit.");
        return;
    }

    let fetch_status = std::process::Command::new("git")
        .arg("fetch")
        .current_dir(notes_home)
        .status();
    
    if let Err(e) = fetch_status {
        eprintln!("Failed to fetch changes: {}", e);
        return;
    }
    
    if fetch_status.unwrap().success() {
        println!("Fetched changes from remote repository.");
    } else {
        eprintln!("Failed to fetch changes. Please check your remote repository configuration.");
        return;
    }

    let push_status = std::process::Command::new("git")
        .arg("push")
        .current_dir(notes_home)
        .status();
    if let Err(e) = push_status {
        eprintln!("Failed to push changes: {}", e);
        return;
    }
    if push_status.unwrap().success() {
        println!("Pushed changes to remote repository.");
    } else {
        eprintln!("Failed to push changes. Please check your remote repository configuration.");
    }
    println!("Sync operation completed successfully.");
}