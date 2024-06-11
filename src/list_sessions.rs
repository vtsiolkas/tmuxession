use crate::common::{capture_session_name_from_script, get_data_dir, get_user_option, UserOption};
use crate::restore_session::restore_tmux_session;
use std::fs;
use urlencoding::decode;

pub fn list_sessions() {
    let session_dir = get_data_dir().to_path_buf();
    let entries = fs::read_dir(&session_dir).expect("Could not read directory");

    let mut session_files = Vec::new();
    let mut session_names = Vec::new();
    for entry in entries {
        let entry = entry.expect("Could not read entry");
        let path = entry.path();
        if path.is_file() {
            if let Some(extension) = path.extension() {
                if extension == "sh" {
                    let shell_script = match fs::read_to_string(&path) {
                        Ok(script) => script,
                        Err(_) => {
                            continue;
                        }
                    };

                    // Read the session_name value
                    let session_name = capture_session_name_from_script(&shell_script);
                    if session_name.is_empty() {
                        continue;
                    }

                    session_names.push(session_name);
                    session_files.push(path);
                }
            }
        }
    }

    if session_files.is_empty() {
        println!("No saved tmux sessions found.");
        return;
    }

    let mut options: Vec<UserOption> = Vec::new();

    for (i, path) in session_files.iter().enumerate() {
        let encoded_name = path.file_stem().unwrap().to_str().unwrap();

        let decoded_name = decode(encoded_name).unwrap();
        let label = format!("[{}] {}: {}", i + 1, &session_names[i], decoded_name);

        options.push(UserOption {
            label: label.to_string(),
            keybind: (i as u8 + '1' as u8) as char,
        });
    }

    options.push(UserOption {
        label: "[q] Quit".to_string(),
        keybind: 'q',
    });

    let choice = get_user_option(
        "Saved sessions in tmuxession data directory\n-------------------------------------------",
        options,
    );

    if choice == 'q' {
        return;
    }
    if let Some(index) = (choice as u8).checked_sub('1' as u8) {
        if let Some(selected_path) = session_files.get(index as usize) {
            let script_path = selected_path.to_str().unwrap().to_string();
            restore_tmux_session(Some(script_path));
        } else {
            println!("Invalid selection.");
        }
    }
}
