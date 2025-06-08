use std::fs;
use clap::{Parser, Subcommand};
use config::config::{create_config_file, read_config_file, Config};
use handlers::new_handler::handle_new_note;
use handlers::log_handler::handle_log;
use handlers::sync_handler::handle_sync;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    cmd: Commands,
}

#[derive(Subcommand,Debug, Clone)]
enum Commands {
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
    let config_contents = init();

    let config = read_config_file(config_contents);

    let app_state: AppState = AppState {
        config,
    };

    run_app(app_state);
}

fn init() -> String {
    let config_directory = format!("/home/{}/.config/devnotes/config.toml", whoami::username());

    // load config file from home
    let config = fs::read_to_string(config_directory.clone());

    match config {
        Ok(config_file) => return config_file,
        Err(error) => {
            println!("Error reading config file: {}", error);
            let _new_config = create_config_file(&config_directory).unwrap();
            fs::read_to_string(config_directory).unwrap()
        }
    }
}

fn run_app(app_state: AppState) {
    let args = Args::parse();

    match args.cmd {
        Commands::New{value} => {
            handle_new_note(app_state, value)
        },
        Commands::Log{value} => {
            handle_log(app_state, value)
        },
        Commands::View{key :_, value: _} => todo!(),
        Commands::Edit{key: _, value: _} => todo!(),
        Commands::Sync{value} => {
            handle_sync(app_state, value)
        }
    }
}