use crate::common::{get_session_script_path, TmuxPane, TmuxSession, TmuxWindow};
use crate::generate_script::generate_tmux_session_script;
use std::process::Command;
use std::{fs, path::PathBuf};

pub fn save_tmux_session(script: Option<String>, provided_session_name: Option<String>) {
    let session_name = match provided_session_name {
        Some(name) => name,
        None => get_tmux_session_name(),
    };

    let windows = get_tmux_windows();
    let session = TmuxSession {
        name: session_name.clone(),
        windows,
    };

    let file_path = match script {
        Some(path) => PathBuf::from(path),
        None => get_session_script_path(),
    };

    let shell_script = generate_tmux_session_script(&session);
    fs::write(&file_path, shell_script).unwrap();

    println!("Tmux session `{}` saved successfully.", &session_name);
    println!(
        "Script for restoring the session saved under: {}",
        &file_path.display()
    );
}

fn get_tmux_session_name() -> String {
    let output = Command::new("tmux")
        .arg("display-message")
        .arg("-p")
        .arg("#S")
        .output()
        .unwrap();
    String::from_utf8_lossy(&output.stdout).trim().to_string()
}

fn get_tmux_windows() -> Vec<TmuxWindow> {
    let output = Command::new("tmux")
        .arg("list-windows")
        .arg("-F")
        .arg("#{window_index}:#{window_name}:#{window_layout}:#{window_active}:#{window_zoomed_flag}")
        .output()
        .unwrap();
    let windows_output = String::from_utf8_lossy(&output.stdout);

    windows_output
        .lines()
        .map(|line| {
            let parts: Vec<&str> = line.split(':').collect();
            let id = parts[0].to_string();
            let name = parts[1].to_string();
            let layout = parts[2].to_string();
            let active = parts[3] == "1";
            let zoomed = parts[4] == "1";
            let panes = get_tmux_panes(&id);

            TmuxWindow {
                id,
                name,
                layout,
                panes,
                active,
                zoomed,
            }
        })
        .collect()
}

fn get_tmux_panes(window_id: &str) -> Vec<TmuxPane> {
    let output = Command::new("tmux")
        .arg("list-panes")
        .arg("-t")
        .arg(window_id)
        .arg("-F")
        .arg("#{pane_index}:#{pane_current_path}:#{pane_pid}:#{pane_active}")
        .output()
        .unwrap();
    let panes_output = String::from_utf8_lossy(&output.stdout);

    panes_output
        .lines()
        .filter_map(|line| {
            let parts: Vec<&str> = line.split(':').collect();
            let pane_id = parts[0].to_string();
            let cwd = parts[1].to_string();
            let pid: i32 = parts[2].parse().unwrap();
            let active = parts[3] == "1";
            let commands = get_full_command(pid);

            Some(TmuxPane {
                id: pane_id,
                cwd,
                active,
                commands,
            })
        })
        .collect()
}

fn sanitize_command_from_ps(stdout: Vec<u8>) -> String {
    let command = String::from_utf8_lossy(&stdout).trim().to_string();

    if command.starts_with("-") {
        command[1..].to_string()
    } else {
        command
    }
}

fn get_full_command(pid: i32) -> Vec<String> {
    let mut commands = vec![];

    // Use ps to get child processes of the given PID
    let output = Command::new("ps")
        .arg("--ppid")
        .arg(pid.to_string())
        .arg("-o")
        .arg("pid=")
        .output()
        .unwrap();
    let child_pids = String::from_utf8_lossy(&output.stdout)
        .lines()
        .map(|line| line.trim().parse::<i32>().unwrap())
        .collect::<Vec<_>>();

    let output = Command::new("ps")
        .arg("-p")
        .arg(pid.to_string())
        .arg("-o")
        .arg("args=")
        .output()
        .unwrap();
    let command = sanitize_command_from_ps(output.stdout);
    commands.push(command);

    // Iterate over child PIDs to find the actual command running in the pane
    for child_pid in child_pids {
        let output = Command::new("ps")
            .arg("-p")
            .arg(child_pid.to_string())
            .arg("-o")
            .arg("args=")
            .output()
            .unwrap();
        let command = sanitize_command_from_ps(output.stdout);

        // Ignore empty commands and tmuxession commands
        if !command.is_empty() && !command.contains("tmuxession") {
            commands.push(command);
            break;
        }
    }

    commands
}
