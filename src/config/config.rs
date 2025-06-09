use std::{fs, io::{Error, Write}};

use crate::fs::File;
use serde::{Deserialize, Serialize};
use toml;

#[derive(Deserialize, Serialize, Debug)]
pub struct Config {
    pub notes_home: String,
    pub repo_url: Option<String>,
    pub sync_enabled: bool,
    pub editor: Option<String>,
}

pub fn create_config_file(path: &String, notes_home: Option<String>, repo_url: Option<String>, sync_enabled: Option<bool>) -> Result<(), Error> {
    // TODO: add multi OS support
    let default_config = Config {
        notes_home: notes_home.unwrap_or(format!("/home/{}/devnotes", whoami::username())),
        repo_url: repo_url,
        sync_enabled: sync_enabled.unwrap_or(false),
        editor: Some(std::env::var("EDITOR").unwrap_or_else(|_| "nano".to_string())),
    };

    let config_directory = format!("/home/{}/.config/devnotes", whoami::username());

    if !std::path::Path::new(&config_directory).exists() {
        let _directory = fs::create_dir(config_directory).unwrap_or_else(|_| {
            println!("Directory already exists");
        });
    }

    let mut toml = String::new();

    match toml::to_string(&default_config) {
        std::result::Result::Ok(toml_string) => {
            toml = toml_string
        }
        Err(error) => {
            eprintln!("Failed to parse config file: {}", error)
        }
    }
    
    let config_file = File::create(path);
    
    match config_file {
        std::result::Result::Ok(mut file) => {
            let file_result = file.write_all(toml.as_bytes());

            match file_result {
                std::result::Result::Ok(file) => {
                    println!("Config file created successfully at: {}", path);
                    std::result::Result::Ok(file)
                },
                Err(error) => {
                    println!("Failed to write to config file: {}", error);
                    Err(error)
                },
            }
        },
        Err(error) => {
            println!("Could not create file: {}", error);
            Err(error)
        }
    }
}

pub fn read_config_file(config_contents: String) -> Option<Config> {
    let config: Option<Config> = toml::from_str(&config_contents).unwrap_or_else(|_| {
        println!("Failed to parse config file. Please check the format.");
        None
    });

    return config;
}