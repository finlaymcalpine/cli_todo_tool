use clap::Parser;
mod cli;
mod tasks;

use cli::{Action::*, Arguments};
use std::path::PathBuf;
use tasks::Task;

fn assign_default_file() -> Option<PathBuf> {
    home::home_dir().map(|mut path| {
        path.push(".rust_journal.json");
        path
    })
}

fn main() {
    // Get the command-line arguments.
    let Arguments { action, task_file } = Arguments::parse();

    // Unpack the journal file.
    let file_path = task_file
        .or_else(assign_default_file)
        .expect("Failed to find journal file");

    // Perform the action.
    match action {
        Add { task } => tasks::add_task(file_path, Task::new(task)),
        List => tasks::list_tasks(file_path),
        Done { position } => tasks::complete_task(file_path, position),
    }
    .expect("Failed to perform action")
}
