use std::path::PathBuf;

use clap::{Parser, Subcommand};

/// A minimal terminal task manager.
#[derive(Parser)]
#[command(name = "tic", version, about)]
pub struct Cli {
    /// Directory where tasks are stored.
    ///
    /// Defaults to a local `.config/tic` directory.
    #[arg(long, global = true)]
    pub path: Option<PathBuf>,

    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    /// Add a new task.
    Add {
        /// The task description.
        text: String,
    },
    /// List all tasks.
    List,
    /// Mark a task as done.
    Done {
        /// The task id.
        id: usize,
    },
    /// Update a task's description.
    Edit {
        /// The task id.
        id: usize,
        /// The new task description.
        text: String,
    },
    /// Remove a task.
    Remove {
        /// The task id.
        id: usize,
    },
    /// Remove all tasks.
    Clear,
}
