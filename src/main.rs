mod common;
mod edit_session_script;
mod generate_script;
mod restore_session;
mod save_session;

use clap::{Parser, Subcommand};
use edit_session_script::edit_session_script;
use restore_session::restore_tmux_session;
use save_session::save_tmux_session;
use std::env;

/// tmuxession: Save and restore tmux sessions
#[derive(Debug, Parser)]
#[command(
    name = "tmuxession",
    version = clap::crate_version!(),
    author = clap::crate_authors!(),
    about = clap::crate_description!()
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Save the current TMUX session
    #[command(alias = "s")]
    Save {
        /// Optional script parameter
        #[arg(long)]
        script: Option<String>,
    },
    /// Restore the TMUX session
    #[command(alias = "r")]
    Restore {
        /// Optional script parameter
        #[arg(long)]
        script: Option<String>,
    },
    /// Edit the saved TMUX session for the current directory
    /// This command will open the saved script if it extists
    /// in $EDITOR or vi
    #[command(alias = "e")]
    Edit {},
}

fn main() {
    let args = Cli::parse();

    let inside_tmux = env::var("TMUX").is_ok()
        || env::var("TERM_PROGRAM").unwrap_or_default() == "tmux"
        || env::var("TMUX_PANE").is_ok();

    match args.command {
        Commands::Save { script } => {
            if !inside_tmux {
                eprintln!("Error: `tmuxession save` must be run inside a tmux session");
                std::process::exit(1);
            }
            save_tmux_session(script);
        }
        Commands::Restore { script } => {
            restore_tmux_session(script);
        }
        Commands::Edit {} => {
            edit_session_script();
        }
    }
}
