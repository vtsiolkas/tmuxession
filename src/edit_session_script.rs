use crate::common::get_session_script_path;
use std::env;

pub fn edit_session_script() {
    let file_path = get_session_script_path();

    if !file_path.exists() {
        eprintln!(
            "Could not find session script for the current directory: {:?}",
            file_path,
        );
        std::process::exit(1);
    }

    let editor = env::var("EDITOR").unwrap_or("vi".to_string());
    std::process::Command::new(editor)
        .arg(file_path)
        .status()
        .expect("Failed to open the editor");
}
