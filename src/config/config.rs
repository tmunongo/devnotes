use std::{fmt::format, fs, io::Write};

use crate::fs::File;
use serde::{Deserialize, Serialize};
use toml;

#[derive(Deserialize, Serialize)]
pub struct Config {
    notes_home: String,
}

pub fn create_config_file(path: String) -> Option<File> {
    let default_config = Config {
        notes_home: path.to_string()
    };

    let config_directory = format!("/home/{}/.config/devnotes", whoami::username());

    let _directory = fs::create_dir(config_directory).unwrap();

    let toml = toml::to_string(&default_config).unwrap();
    
    let config_file = File::create(path);
    
    match config_file {
        Ok(mut file) => {
            file.write_all(toml.as_bytes()).unwrap();
            Some(file)
        },
        Err(error) => {
            println!("Could not create file: {}", error);
            None
        }
    }
}

pub fn read_config_file(config_contents: String) {
    let config: Config = toml::from_str(&config_contents).unwrap();

    println!("Config notes home: {}", config.notes_home);
}