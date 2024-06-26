use crate::common::TmuxSession;

pub fn generate_tmux_session_script(session: &TmuxSession) -> String {
    let session_name = session.name.clone();
    let mut shell_script = String::new();

    // Create the session
    shell_script.push_str("#!/bin/bash\n\n");
    shell_script.push_str("###############################################\n");
    shell_script.push_str("# Script generated by tmuxession              #\n");
    shell_script.push_str("###############################################\n\n");
    shell_script.push_str("# Exit on error or unset variable\n");
    shell_script.push_str("set -e\nset -u\n\n\n");
    shell_script.push_str("# Session name\n");
    shell_script.push_str(format!("session_name={}\n\n\n", &session_name).as_str());
    shell_script.push_str("### Create a new detached tmux session\n");
    shell_script.push_str(&format!("tmux new-session -d -s \"$session_name\"\n\n\n"));

    let mut active_pane = String::new();
    let mut active_window = String::new();
    let mut zoomed_panes: Vec<String> = vec![];
    for window in &session.windows {
        shell_script.push_str(format!("## Window {}:{}\n", &window.id, &window.name).as_str());
        let target_window = format!("\"$session_name\":{}", &window.id);
        shell_script.push_str(&format!(
            "tmux new-window -t {} -k -n {} -c {} \"{}\"\n\n",
            &target_window, &window.name, &window.panes[0].cwd, &window.panes[0].commands[0]
        ));

        let mut active_pane_current_window = String::new();
        for (i, pane) in window.panes.iter().enumerate() {
            if i != 0 {
                // Create a new pane and run the first command in it
                shell_script.push_str(format!("# Create pane {}\n", &pane.id).as_str());
                shell_script.push_str(&format!(
                    "tmux split-window -t {} -c {} \"{}\"\n",
                    &target_window, &pane.cwd, &pane.commands[0]
                ));
            }

            let target_pane = format!("\"$session_name\":{}.{}", &window.id, pane.id);
            if window.active && pane.active {
                // Set the active pane to select it at the end
                active_pane = format!("tmux select-pane -t {}\n\n", &target_pane);
            } else if pane.active {
                active_pane_current_window = format!("tmux select-pane -t {}\n\n", &target_pane);
            }

            if pane.active && window.zoomed {
                zoomed_panes.push(format!("tmux resize-pane -t {} -Z\n", &target_pane));
            }

            // Run the second command in the pane
            for command in &pane.commands[1..] {
                shell_script.push_str(format!("# Run command in pane {}\n", &pane.id).as_str());
                shell_script.push_str(&format!(
                    "tmux send-keys -t {} \"{}\" C-m\n\n",
                    &target_pane, command
                ));
            }
        }

        // Select the active pane in the current window
        shell_script
            .push_str(format!("# Select the active pane in window {}\n", &window.name).as_str());
        shell_script.push_str(&active_pane_current_window);

        if window.active {
            // Set the active window to select it at the end
            active_window = format!("tmux select-window -t {}\n\n", &target_window);
        }

        // Set the layout of the window
        shell_script.push_str(format!("# Set layout for window {}\n", &window.name).as_str());
        shell_script.push_str(&format!(
            "tmux select-layout -t {} \"{}\"\n\n",
            &target_window, &window.layout
        ));
        shell_script
            .push_str(format!("## End of window {}:{}\n\n", &window.id, &window.name).as_str());
    }

    if !zoomed_panes.is_empty() {
        shell_script.push_str("### Zoom the zoomed panes\n");
        for zoomed_pane in zoomed_panes {
            shell_script.push_str(&zoomed_pane);
        }
    }

    // Select the active window
    shell_script.push_str("### Select the active window\n");
    shell_script.push_str(&active_window);
    // Select the active pane
    shell_script.push_str("### Select the active pane\n");
    shell_script.push_str(&active_pane);

    shell_script
}
