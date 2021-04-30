use anyhow::anyhow;
use cli::{Action::*, CommandLineArgs};
use std::path::PathBuf;
use structopt::StructOpt;

mod cli;
mod tasks;

fn find_default_task_file() -> Option<PathBuf> {
    home::home_dir().map(|mut path| {
        path.push(".task_file.json");
        path
    })
}

fn main() -> anyhow::Result<()> {
    let CommandLineArgs { action, task_file } = CommandLineArgs::from_args();

    let task_file = task_file
        .or_else(find_default_task_file)
        .ok_or(anyhow!("Failed to find task file !!"))?;

    match action {
        Add { text } => tasks::add_task(task_file, tasks::Task::new(text)),
        List => tasks::list_tasks(task_file),
        Done { position } => tasks::complete_task(task_file, position),
    }?;

    Ok(())
}
