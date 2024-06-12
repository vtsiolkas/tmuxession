mod common;
mod edit_session_script;
mod generate_script;
mod list_sessions;
mod restore_session;
mod save_session;
mod tmux_commands;
mod ui;

use crate::common::is_inside_tmux;
use crate::list_sessions::list_sessions;
use clap::{Parser, Subcommand};
use edit_session_script::edit_session_script;
use restore_session::restore_tmux_session;
use save_session::save_tmux_session;
use std::env;

/// tmuxession: Save and restore tmux sessions.
#[derive(Debug, Parser)]
#[command(
    name = "tmuxession",
    version = clap::crate_version!(),
    author = clap::crate_authors!(),
    about = clap::crate_description!()
)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Save the current TMUX session
    #[command(visible_alias = "s")]
    Save {
        /// Optional session script parameter.
        #[arg(long)]
        script: Option<String>,
        /// Optional session name parameter.
        /// If not provided, the current session name will be used.
        #[arg(long)]
        name: Option<String>,
    },
    /// Restore the TMUX session
    #[command(visible_alias = "r")]
    Restore {
        /// Optional script parameter
        #[arg(long)]
        script: Option<String>,
    },
    /// Edit the saved TMUX session for the current directory.
    /// This command will open the saved script if it exists
    /// in $EDITOR or vi
    #[command(visible_alias = "e")]
    Edit {},
    /// List all saved TMUX sessions and allows to pick one to restore [default]
    #[command(visible_alias = "l")]
    List {},
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(command) => match command {
            Commands::Save { script, name } => {
                if !is_inside_tmux() {
                    eprintln!("Error: `tmuxession save` must be run inside a tmux session");
                    std::process::exit(1);
                }
                save_tmux_session(script.clone(), name.clone());
            }
            Commands::Restore { script } => {
                restore_tmux_session(script.clone());
            }
            Commands::Edit {} => {
                edit_session_script();
            }
            Commands::List {} => {
                list_sessions();
            }
        },
        None => {
            list_sessions();
        }
    }
}
