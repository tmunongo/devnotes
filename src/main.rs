use std::fs;
use clap::{Parser, Subcommand};
use color_eyre::eyre::eyre;
use config::config::{create_config_file, read_config_file, Config};
use handlers::new_handler::handle_new_note;

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
    }
}

#[derive(Debug)]
pub struct AppState {
    config: Config,
}

mod config;
mod handlers;

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
        Commands::View{key, value} => todo!(),
        Commands::Edit{key, value} => todo!(),
    }
}