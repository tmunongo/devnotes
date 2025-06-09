use crate::config::config::{create_config_file};

pub fn handle_init(value: Option<String>) {
    if let Some(val) = value {
        let val = val.to_lowercase();
        if val == "help" || val == "h" {
            println!("Usage: devnotes init [existing|e|new|n]");
            println!("Initialize the devnotes application.");
            println!("Options:");
            println!("  existing, e: Use an existing git repository for syncing notes.");
            println!("  new, n: Initialise a new git repository for syncing notes.");
            return;
        }

        println!("Please enter the path to your preferred notes path (default: /home/{}/Documents/devnotes):", whoami::username());
        let mut notes_home = String::new();
        std::io::stdin().read_line(&mut notes_home).expect("Failed to read line");
        let notes_home = notes_home.trim();
        if notes_home.is_empty() {
            println!("No path provided, using default: /home/{}/Documents/devnotes", whoami::username());
        } else {
            println!("Using provided path: {}", notes_home);
        }

        let notes_home = if notes_home.is_empty() {
            format!("/home/{}/Documents/devnotes", whoami::username())
        } else {
            notes_home.to_string()
        };
        if !std::path::Path::new(&notes_home).exists() {
            std::fs::create_dir_all(&notes_home).unwrap_or_else(|_| {
                println!("Failed to create notes directory. Please check the permissions and try again.");
                return;
            });
            println!("Created notes directory at: {}", notes_home);
        } else {
            println!("Using existing notes directory at: {}", notes_home);
        }

        let mut repo_url: String = String::new();
        if val == "existing" || val == "e" {
            println!("Please enter the URL of your existing git repository: ",);
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).expect("Failed to read line");
            let input = input.trim();
            if input.is_empty() {
                println!("No path to a repository provided.",);
            } else {
                repo_url = input.to_string();
                // clone the github repo to notes_home
                println!("Using provided path: {}", input);
            }
        } else {
            // TODO: init a new repo
        }

        let config_path = format!("/home/{}/.config/devnotes/devnotes.toml", whoami::username());
        
        let config_file = create_config_file(
            &config_path,
            Some(notes_home),
            Some(repo_url),
            Some(true),
        );

        match config_file {
            Ok(()) => {
                println!("Configuration file created successfully at: {}", config_path);
                println!("You can now start using devnotes. Use 'devnotes help' to see available commands.");
            }
            Err(error) => {
                println!("Failed to create your config file because: {}", error)
            }
        }
    } else {
        eprintln!("Usage: devnotes init [existing|e|new|n]");
        return;
    }
}