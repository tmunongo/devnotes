use crate::AppState;

pub fn sanitize_file_name(path: &String) -> String {
    path.replace("/", "_")
}

pub fn logfile_in_path(app_state: AppState, path: &String) -> String {

    format!("{}/{}", app_state.config.notes_home, sanitize_file_name(path))
}

pub fn get_subdirectories_from_date(date: &String) -> Vec<&str> {
    let parts = date.split('-').collect::<Vec<&str>>();

    parts
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sanitize_file_name() {
        let path = "some/file/name".to_string();

        assert_eq!("some_file_name".to_string(), sanitize_file_name(&path));
    }
}