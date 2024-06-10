use crate::common::{get_session_script_path, run_tmux_command};
use crossterm::{
    event::{self, Event, KeyCode},
    terminal,
};
use regex::Regex;
use std::fs;
use std::io::stdout;
use std::io::{self, Write};
use std::path::PathBuf;
use std::process::Command;

pub fn restore_tmux_session(script: Option<String>) {
    let file_path = match script {
        Some(path) => PathBuf::from(path),
        None => get_session_script_path(),
    };

    if !file_path.exists() {
        eprintln!(
            "The provided path does not exist or is not a file: {:?}",
            file_path
        );
        std::process::exit(1);
    }

    let mut shell_script = fs::read_to_string(file_path).unwrap();
    let re = Regex::new(r"(?m)^session_name=(.*)$").unwrap();
    // Read the session_name value
    let mut session_name = re
        .captures(&shell_script)
        .and_then(|caps| caps.get(1))
        .map_or("", |m| m.as_str().trim())
        .to_string();

    if session_name.is_empty() {
        eprintln!("Error: Could not find session_name variable in the script");
        std::process::exit(1);
    }
    println!("Restoring tmux session {}...", &session_name);

    while check_session_exists(&session_name) {
        let user_option = get_user_option();
        terminal::disable_raw_mode().unwrap();
        print!("\r");
        match user_option {
            'A' => {
                if check_session_attached(&session_name) {
                    let attached_session_option = get_attached_session_option();
                    terminal::disable_raw_mode().unwrap();
                    print!("\r");
                    match attached_session_option {
                        'A' => {}
                        'q' => {
                            println!("Exiting without restoring the session.");
                            std::process::exit(0);
                        }
                        _ => unreachable!(),
                    }
                }

                let args = vec!["attach", "-t", &session_name];
                let mut tmux = Command::new("tmux")
                    .args(&args)
                    .spawn()
                    .expect("Failed to spawn process");

                let _ = tmux.wait();
                std::process::exit(0);
            }
            'K' => {
                let args = vec!["kill-session", "-t", &session_name];
                run_tmux_command(&args);
            }
            'R' => {
                let mut new_name = String::new();
                print!("Enter new session name: ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut new_name).unwrap();
                session_name = new_name.trim().to_string();
                shell_script = re
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

    // Restore the session
    let mut tmux = Command::new("sh")
        .arg("-c")
        .arg(&shell_script)
        .spawn()
        .expect("Failed to spawn process");

    let _ = tmux.wait();
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

fn get_user_option() -> char {
    let mut stdout = stdout();
    println!("A session with the same name already exists.\n");
    println!("[A]ttach to existing session");
    println!("[K]ill existing session and replace");
    println!("[R]estore with a different session name");
    println!("[q]uit:");
    print!("Enter your choice: ");
    // Enable raw mode to read input immediately
    terminal::enable_raw_mode().unwrap();

    loop {
        stdout.flush().unwrap();

        // Read input immediately on key press
        if let Event::Key(key_event) = event::read().unwrap() {
            match key_event.code {
                KeyCode::Char('A') => {
                    print!("A\n");
                    return 'A';
                }
                KeyCode::Char('K') => {
                    print!("K\n");
                    return 'K';
                }
                KeyCode::Char('R') => {
                    print!("R\n");
                    return 'R';
                }
                KeyCode::Char('q') => {
                    print!("q\n");
                    return 'q';
                }
                _ => {}
            }
        }
    }
}

fn get_attached_session_option() -> char {
    let mut stdout = stdout();
    println!("The session is already attached in another client.\n");
    println!("[A]ttach here anyway");
    println!("[q]uit:");
    print!("Enter your choice: ");
    // Enable raw mode to read input immediately
    terminal::enable_raw_mode().unwrap();

    loop {
        stdout.flush().unwrap();

        // Read input immediately on key press
        if let Event::Key(key_event) = event::read().unwrap() {
            match key_event.code {
                KeyCode::Char('A') => {
                    println!("A\n");
                    return 'A';
                }
                KeyCode::Char('q') => {
                    print!("q\n");
                    return 'q';
                }
                _ => {}
            }
        }
    }
}

fn check_session_attached(session_name: &str) -> bool {
    let output = Command::new("tmux")
        .args(&["list-clients", "-F", "#{session_name}"])
        .output()
        .expect("Failed to execute tmux command");

    if !output.stderr.is_empty() {
        let stderr = std::str::from_utf8(&output.stderr).expect("Failed to read stderr");
        println!("Error: {}", stderr);
        std::process::exit(1);
    }

    let stdout = std::str::from_utf8(&output.stdout).expect("Failed to read stdout");
    stdout.lines().any(|line| line == session_name)
}
