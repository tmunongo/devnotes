use std::fs;
use clap::{Parser, Subcommand};
use config::config::create_config_file;
use handlers::handlers::handle_new_note;

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

mod config;
mod handlers;

fn main() {
    let args = Args::parse();

    // TODO: extract config creation/reading into an init function

    let config_directory = format!("/home/{}/.config/devnotes/config.toml", whoami::username());

    // load config file from home
    let config = fs::read_to_string(config_directory.clone());

    match config {
        Ok(config_file) => println!("{}", config_file),
        Err(error) => {
            println!("Error reading config file: {}", error);
            create_config_file(config_directory);
        }
    }

    match args.cmd {
        Commands::New{value} => {
            handle_new_note(value)
        },
        Commands::View{key, value} => todo!(),
        Commands::Edit{key, value} => todo!(),
    }
}