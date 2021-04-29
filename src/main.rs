mod cli;
use structopt::StructOpt;
mod tasks;

use cli::{Action::*, CommandLineArgs};

fn main() -> Result<(), std::io::Error> {
    let CommandLineArgs { action, task_file } = CommandLineArgs::from_args();

    let task_file = task_file.expect("Failed to find task file !!");

    match action {
        Add { text } => tasks::add_task(task_file, tasks::Task::new(text)),
        List => tasks::list_tasks(task_file),
        Done { position } => tasks::complete_task(task_file, position),
    }
}
