use std::{env, fs::{self, File}};

use crate::AppState;
use crate::helpers::sanitize_file_name;

pub fn handle_new_note(app_state: AppState, mut path: String) {
    // check that the supplied path is a valid file name
    path = sanitize_file_name(&path);

    let new_note_path = app_state.config.notes_home.to_owned() + "/" + &path  + ".md";

    // TODO: check if file doesn't exist

    let create_file_result = File::create(&new_note_path);

    match create_file_result {
        Ok(_file) => println!("Successfuly created your new markdown note"),
        Err(_error) => {
            println!("Creating devnotes directory...");
            fs::create_dir(&app_state.config.notes_home).expect("Could not create your devnotes directory");
            // try to create again
            File::create(&new_note_path).expect("Could not create note, again! :(");
        }
    }

    let editor = env::var("EDITOR").unwrap_or_else(|_| "nvim".to_string());
    open::with(&new_note_path, &editor)
        .expect("Could not open this note");
}