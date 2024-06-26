use crate::tmux_commands::get_current_pane_cwd;
use lazy_static::lazy_static;
use regex::Regex;
use std::env;
use std::path::PathBuf;
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
    pub zoomed: bool,
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

pub fn get_data_dir() -> PathBuf {
    let xdg_dirs = BaseDirectories::with_prefix("tmuxession").unwrap();
    let data_dir = xdg_dirs.get_data_home();
    std::fs::create_dir_all(&data_dir).expect("Could not create tmuxession data directory");
    data_dir
}

pub fn get_session_script_path() -> PathBuf {
    let xdg_dirs = BaseDirectories::with_prefix("tmuxession").unwrap();
    let current_dir = match is_inside_tmux() {
        true => get_current_pane_cwd(),
        false => env::current_dir().unwrap().to_string_lossy().to_string(),
    };
    let file_name = encode(&current_dir);
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
