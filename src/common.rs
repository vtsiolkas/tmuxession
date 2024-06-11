use crossterm::event::{self, Event, KeyCode};
use crossterm::terminal;
use lazy_static::lazy_static;
use regex::Regex;
use std::env;
use std::io::{stdout, Write};
use std::path::PathBuf;
use std::process::Command;
use urlencoding::encode;
use xdg::BaseDirectories;

#[derive(Debug)]
pub struct TmuxPane {
    pub id: String,
    pub cwd: String,
    pub active: bool,
    pub commands: Vec<String>,
}

#[derive(Debug)]
pub struct TmuxWindow {
    pub id: String,
    pub name: String,
    pub layout: String,
    pub active: bool,
    pub panes: Vec<TmuxPane>,
}

#[derive(Debug)]
pub struct TmuxSession {
    pub name: String,
    pub windows: Vec<TmuxWindow>,
}

#[derive(Debug)]
pub struct UserOption {
    pub label: String,
    pub keybind: char,
}

lazy_static! {
    pub static ref TMUX_SESSION_RE: Regex = Regex::new(r"(?m)^session_name=(.*)$").unwrap();
}

pub fn get_user_option(title: &str, options: Vec<UserOption>) -> char {
    let mut stdout = stdout();
    println!("{}", title);

    for option in &options {
        println!("{}", option.label);
    }

    print!("Enter your choice: ");
    terminal::enable_raw_mode().unwrap();

    let char;

    loop {
        stdout.flush().unwrap();

        if let Event::Key(key_event) = event::read().unwrap() {
            if let KeyCode::Char(c) = key_event.code {
                if let Some(option) = options.iter().find(|o| o.keybind == c) {
                    print!("{}\n", option.keybind);
                    char = c;
                    break;
                }
            }
        }
    }
    terminal::disable_raw_mode().unwrap();
    print!("\r");

    char
}

pub fn run_tmux_command(args: &[&str]) {
    let output = Command::new("tmux")
        .args(args)
        .output()
        .expect("Failed to execute tmux command");

    if !output.stderr.is_empty() {
        let stderr = std::str::from_utf8(&output.stderr).expect("Failed to read stderr");
        eprintln!("Error: {}", stderr);
        std::process::exit(1);
    }

    // let stdout = std::str::from_utf8(&output.stdout).expect("Failed to read stdout");
    // println!("Command: {}, Output: {}", args.join(" "), stdout);
}

pub fn get_data_dir() -> PathBuf {
    let xdg_dirs = BaseDirectories::with_prefix("tmuxession").unwrap();
    xdg_dirs.get_data_home()
}

pub fn get_session_script_path() -> PathBuf {
    let xdg_dirs = BaseDirectories::with_prefix("tmuxession").unwrap();
    let current_dir = env::current_dir().unwrap();
    let file_name = encode(current_dir.to_str().unwrap()).to_string();
    xdg_dirs
        .place_data_file(format!("{}.sh", file_name))
        .unwrap()
}

pub fn is_inside_tmux() -> bool {
    let inside_tmux = env::var("TMUX").is_ok()
        || env::var("TERM_PROGRAM").unwrap_or_default() == "tmux"
        || env::var("TMUX_PANE").is_ok();

    inside_tmux
}

pub fn capture_session_name_from_script(shell_script: &str) -> String {
    // Return the session_name value
    TMUX_SESSION_RE
        .captures(&shell_script)
        .and_then(|caps| caps.get(1))
        .map_or("", |m| m.as_str().trim())
        .to_string()
}
