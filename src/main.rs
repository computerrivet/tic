mod cli;
mod store;
mod task;

use anyhow::Result;
use clap::Parser;

use cli::{Cli, Command};
use task::format_relative;

fn main() -> Result<()> {
    let cli = Cli::parse();

    let mut store = store::Store::open(cli.path)?;

    match cli.command {
        Command::Add { text } => {
            let task = store.add(text)?;
            println!("Added task #{}: {}", task.id, task.text);
        }
        Command::List => {
            let tasks = store.list()?;
            if tasks.is_empty() {
                println!("No tasks.");
            } else {
                let now = time::OffsetDateTime::now_utc().unix_timestamp();
                for task in tasks {
                    let mark = if task.done { "x" } else { " " };
                    let age = format_relative(task.created_at, now);
                    println!("[{}] #{} {} ({})", mark, task.id, task.text, age);
                }
            }
        }
        Command::Done { id } => {
            store.done(id)?;
            println!("Marked task #{} as done.", id);
        }
        Command::Edit { id, text } => {
            store.edit(id, text)?;
            println!("Updated task #{}.", id);
        }
        Command::Remove { id } => {
            store.remove(id)?;
            println!("Removed task #{}.", id);
        }
        Command::Clear => {
            store.clear()?;
            println!("Cleared all tasks.");
        }
    }

    Ok(())
}
