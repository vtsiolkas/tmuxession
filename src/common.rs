use std::env;
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

    let stdout = std::str::from_utf8(&output.stdout).expect("Failed to read stdout");
    println!("Command: {}, Output: {}", args.join(" "), stdout);
}

fn sanitize_path(path: &str) -> String {
    encode(path).to_string()
}

pub fn get_session_script_path() -> PathBuf {
    let xdg_dirs = BaseDirectories::with_prefix("tmuxession").unwrap();
    let current_dir = env::current_dir().unwrap();
    let file_name = sanitize_path(current_dir.to_str().unwrap());
    xdg_dirs
        .place_data_file(format!("{}.sh", file_name))
        .unwrap()
}
