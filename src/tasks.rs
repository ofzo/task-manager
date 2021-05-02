use chrono::{serde::ts_seconds, DateTime, Local, Utc};
use serde::{Deserialize, Serialize};
use serde_json;
use std::fmt;
use std::fs::{File, OpenOptions};
use std::io::{Error, ErrorKind, Result, Seek, SeekFrom};
use std::path::PathBuf;

#[derive(Debug, Deserialize, Serialize)]
pub struct Task {
    pub text: String,

    pub parity: i32,
    #[serde(with = "ts_seconds")]
    pub create_time: DateTime<Utc>,
}

impl Task {
    pub fn new(text: String, parity: i32) -> Task {
        let create_time: DateTime<Utc> = Utc::now();
        Task {
            text,
            create_time,
            parity,
        }
    }
}

impl fmt::Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let create_at = self.create_time.with_timezone(&Local).format("%F %H:%M");
        write!(f, "[{:>4}] {:<50} [{}]", self.parity, self.text, create_at)
    }
}

pub fn add_task(file_path: PathBuf, task: Task) -> Result<()> {
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(file_path)?;

    let mut tasks = collect_task(&file)?;

    tasks.push(task);

    serde_json::to_writer(file, &tasks)?;

    Ok(())
}

pub(crate) fn complete_task(file_path: PathBuf, position: usize) -> Result<()> {
    let file = OpenOptions::new().read(true).write(true).open(file_path)?;

    let mut tasks = collect_task(&file)?;

    if position == 0 || position > tasks.len() {
        return Err(Error::new(ErrorKind::InvalidInput, "Invalid Task ID"));
    }

    tasks.remove(position - 1);

    file.set_len(0)?;

    serde_json::to_writer(file, &tasks)?;

    Ok(())
}

pub fn list_tasks(file_path: PathBuf) -> Result<()> {
    let file = OpenOptions::new().read(true).open(file_path)?;

    let tasks = collect_task(&file)?;

    if tasks.is_empty() {
        println!("Task list is empty!!");
    } else {
        let mut order: u32 = 1;
        for task in tasks {
            println!("{}: {}", order, task);
            order += 1;
        }
    }

    Ok(())
}

fn collect_task(mut file: &File) -> Result<Vec<Task>> {
    file.seek(SeekFrom::Start(0))?;
    let tasks = match serde_json::from_reader(file) {
        Ok(tasks) => tasks,
        Err(e) if e.is_eof() => Vec::new(),
        Err(e) => Err(e)?,
    };

    file.seek(SeekFrom::Start(0))?;

    Ok(tasks)
}
