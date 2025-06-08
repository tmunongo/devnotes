use std::fs;
use std::process::Command;
use crate::AppState;

pub fn handle_log(app_state: AppState, date: Option<String>) {
    match date {
        Some(date) => {
            // TODO: check that date has format YYYY-MM-DD

            let parts = date.split('-').collect::<Vec<&str>>();

            let notes_home = app_state.config.notes_home.clone();

            let year = parts[0].parse::<i32>().unwrap().to_string();
            let month = parts[1].parse::<u32>().unwrap().to_string();

            let year_path = format!("{}/{}", notes_home, year);
            let month_path = format!("{}/{}/{}", notes_home, year, month);
            let log_file = format!("{}/{}.log", month_path, date);

            if !fs::exists(year_path.clone()).unwrap() {
                std::fs::create_dir_all(&year_path).unwrap();
            }

            if !fs::exists(month_path.clone()).unwrap() {
                std::fs::create_dir_all(&month_path).unwrap();
            }

            let editor = std::env::var("EDITOR").unwrap_or_else(|_| "nano".to_string()); // Fallback to nano if EDITOR is not set

            let mut command = Command::new(editor);
            command.arg(log_file);

            let mut child = command.spawn().unwrap();

            let _result = child.wait().unwrap();

            println!("Editor closed.");
        },
        _ => {
            let today = chrono::Utc::now().date_naive();

            println!("today: {}", today);
        }
    }
}

fn location_check(date: String) -> Result<(), bool> {
    todo!()
}

fn create_subdirectories_for_date(date: String) {
    todo!()
}

fn open_in_editor(path: String) -> String {
    let editor = std::env::var("EDITOR").unwrap_or_else(|_| "nano".to_string());

    format!("{} {}", editor, path)
}