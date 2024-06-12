use crate::common::{
    capture_session_name_from_script, get_session_script_path, is_inside_tmux, UserOption,
    TMUX_SESSION_RE,
};
use crate::tmux_commands::{
    attach_session, get_current_tmux_session, kill_session, switch_session,
};
use crate::ui::get_user_option;
use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;
use std::process::Command;

pub fn restore_tmux_session(script: Option<String>) {
    let file_path = match script {
        Some(path) => PathBuf::from(path),
        None => get_session_script_path(),
    };

    let mut shell_script = fs::read_to_string(&file_path)
        .expect(format!("Could not read file: {:?}", &file_path).as_str());

    // Read the session_name value
    let mut session_name = capture_session_name_from_script(&shell_script);
    if session_name.is_empty() {
        eprintln!("Error: Could not find session_name variable in the script");
        std::process::exit(1);
    }

    while check_session_exists(&session_name) {
        let user_option = get_session_exists_option(&session_name);

        match user_option {
            'A' => {
                if is_inside_tmux() {
                    switch_session(&session_name);
                } else {
                    attach_session(&session_name);
                }
            }
            'K' => {
                let current_session_name = get_current_tmux_session();
                if let Some(current_session_name) = current_session_name {
                    if current_session_name == session_name {
                        println!("You are currently inside the session you are trying to kill.");
                        println!("Try doing this from a different session or from outside tmux.");
                        std::process::exit(1);
                    }
                }
                kill_session(&session_name);
            }
            'R' => {
                let mut new_name = String::new();
                print!("Enter new session name: ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut new_name).unwrap();
                session_name = new_name.trim().to_string();
                shell_script = TMUX_SESSION_RE
                    .replace(&shell_script, |_: &regex::Captures| {
                        format!("session_name={}", session_name)
                    })
                    .to_string();
            }
            'q' => {
                println!("Exiting without restoring the session.");
                std::process::exit(0);
            }
            _ => unreachable!(),
        }
    }

    println!("Restoring tmux session \"{}\"...", &session_name);

    // Run the script to restore the session detached
    let mut script_execution = Command::new("sh")
        .arg("-c")
        .arg(&shell_script)
        .spawn()
        .expect("Failed to run shell script");

    let _ = script_execution
        .wait()
        .expect("Failed to wait for session restore script to finish");

    if is_inside_tmux() {
        switch_session(&session_name);
        std::process::exit(0);
    } else {
        attach_session(&session_name);
    }
}

fn check_session_exists(session_name: &str) -> bool {
    let output = Command::new("tmux")
        .args(&["list-sessions", "-F", "#{session_name}"])
        .output()
        .expect("Failed to execute tmux command");

    if !output.stderr.is_empty() {
        let stderr = std::str::from_utf8(&output.stderr).expect("Failed to read stderr");
        // If tmux server is not running, then the session does not exist
        // return false and continue execution
        if stderr.contains("no server running") {
            return false;
        }
        // Else something that we don't know happened, exit just to be sure
        println!("Error: {}", stderr);
        std::process::exit(1);
    }

    let stdout = std::str::from_utf8(&output.stdout).expect("Failed to read stdout");
    stdout.lines().any(|line| line == session_name)
}

fn get_session_exists_option(session_name: &str) -> char {
    let title = format!(
        "A session with the name \"{}\" already exists in the tmux server.",
        &session_name
    );
    let options = vec![
        UserOption {
            keybind: 'A',
            label: "[A]ttach or switch to existing session".to_string(),
        },
        UserOption {
            keybind: 'K',
            label: "[K]ill existing session and replace".to_string(),
        },
        UserOption {
            keybind: 'R',
            label: "[R]estore with a different session name".to_string(),
        },
        UserOption {
            keybind: 'q',
            label: "[q]uit".to_string(),
        },
    ];

    get_user_option(&title, options)
}
