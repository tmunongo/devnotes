use std::fs;
use clap::{Parser, Subcommand};
use config::config::{read_config_file, Config};
use handlers::new_handler::handle_new_note;
use handlers::log_handler::handle_log;
use handlers::sync_handler::handle_sync;

use crate::handlers::init_handler::handle_init;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    cmd: Commands,
}

#[derive(Subcommand,Debug, Clone)]
enum Commands {
    Init {
        value: Option<String>,
    },
    New {
        value: String,
    },
    View {
        key: String,
        value: String,
    },
    Edit {
        key: String,
        value: String,
    },
    Log {
        value: Option<String>,
    },
    Sync {
        value: Option<String>,
    },
}

#[derive(Debug)]
pub struct AppState {
    config: Config,
}

mod config;
mod handlers;
mod helpers;

fn main() {
    let args = Args::parse();

    if let Commands::Init { value } = args.cmd {
        handle_init(value);
        return;
    }

    let config_directory = format!("/home/{}/.config/devnotes/config.toml", whoami::username());
    let config_contents = match fs::read_to_string(&config_directory) {
        Ok(contents) => contents,
        Err(_) => {
            eprintln!("Configuration not found. Please run `devnotes init` to get started.");
            return;
        }
    };

    let config = match read_config_file(config_contents) {
        Some(conf) => conf,
        None => {
            eprintln!("Failed to parse configuration file.");
            return;
        }
    };

    let app_state = AppState { config };

    match args.cmd {
        Commands::New { value } => handle_new_note(app_state, value),
        Commands::Log { value } => handle_log(app_state, value),
        Commands::Sync { value } => handle_sync(app_state, value),
        Commands::View { key: _, value: _ } => todo!(),
        Commands::Edit { key: _, value: _ } => todo!(),
        Commands::Init { .. } => unreachable!(),
    }
}