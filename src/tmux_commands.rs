use crate::common::run_tmux_command;
use std::process::Command;

pub fn attach_session(session_name: &str) {
    let args = vec!["attach", "-t", &session_name];
    let mut tmux = Command::new("tmux")
        .args(&args)
        .spawn()
        .expect("Failed to spawn process");

    let _ = tmux.wait();
    std::process::exit(0);
}

pub fn kill_session(session_name: &str) {
    let args = vec!["kill-session", "-t", &session_name];
    run_tmux_command(&args);
}

pub fn switch_session(session_name: &str) {
    let args = vec!["switch", "-t", &session_name];
    run_tmux_command(&args);
}
