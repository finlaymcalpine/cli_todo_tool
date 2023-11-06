use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Subcommand, Clone)]
pub enum Action {
    Add { task: String },
    //Remove { task: String },
    //Edit { task: String, new_task: String },
    Done { position: usize },
    List,
}

/// Apply defined action to a file of tasks
#[derive(Parser)]
#[command(author="Finlay McAlpine", version="0.1", about="ToDo List CLI", long_about = None)]
pub struct Arguments {
    /// Action to perform
    #[command(subcommand)]
    pub action: Action,
    /// Optional file to retrieve/store tasks (non-default)
    pub task_file: Option<PathBuf>,
}
