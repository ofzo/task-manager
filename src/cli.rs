use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub enum Action {
    /// Write tasks to the task file
    Add {
        /// The task description text.
        #[structopt()]
        text: String,
    },
    /// Remove an entry from the task file by position.
    Done {
        #[structopt()]
        position: usize,
    },
    /// List all tasks in the task file.
    List,
}

#[derive(Debug, StructOpt)]
#[structopt(
    name = "Rusty Task",
    about = "A Command line to-do app written in Rust."
)]
pub struct CommandLineArgs {
    #[structopt(subcommand)]
    pub action: Action,
    #[structopt(parse(from_os_str), short, long)]
    pub task_file: Option<PathBuf>,
}
