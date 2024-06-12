use std::process::Command;

pub fn attach_session(session_name: &str) {
    let args = vec!["attach", "-t", &session_name];
    let mut tmux = Command::new("tmux")
        .args(&args)
        .spawn()
        .expect("Failed to spawn process");

    let _ = tmux.wait().expect("Failed to wait on tmux process");
    std::process::exit(0);
}

pub fn kill_session(session_name: &str) {
    let args = vec!["kill-session", "-t", &session_name];
    let mut tmux = Command::new("tmux")
        .args(&args)
        .spawn()
        .expect("Failed to spawn process");

    let _ = tmux.wait().expect("Failed to wait on tmux process");
}

pub fn switch_session(session_name: &str) {
    let args = vec!["switch-client", "-t", &session_name];
    Command::new("tmux")
        .args(&args)
        .spawn()
        .expect("Failed to spawn process");

    std::process::exit(0);
}

pub fn get_current_pane_cwd() -> String {
    let output = Command::new("tmux")
        .arg("display-message")
        .arg("-p")
        .arg("-F")
        .arg("#{pane_current_path}")
        .output()
        .expect("Failed to execute tmux command");

    String::from_utf8_lossy(&output.stdout).trim().to_string()
}

pub fn get_current_tmux_session() -> Option<String> {
    let output = Command::new("tmux")
        .arg("display-message")
        .arg("-p")
        .arg("#S")
        .output()
        .expect("Failed to execute tmux command");

    if output.status.success() {
        let session_name = String::from_utf8_lossy(&output.stdout).trim().to_string();
        if session_name.is_empty() {
            None
        } else {
            Some(session_name)
        }
    } else {
        None
    }
}
