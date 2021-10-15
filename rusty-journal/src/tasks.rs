use chrono::{serde::ts_seconds, DateTime, Local, Utc};

// Traits that allow de-/serialization
use serde::Deserialize; // get data into the program
use serde::Serialize; // save data outside the program (persistent storage)

use std::io::Result;
use std::path::PathBuf;
use std::fs::File;
use std::io::Read;


#[derive(Debug, Deserialize, Serialize)]
pub struct Task {
    pub text: String,

    #[serde(with = "ts_seconds")]
    pub created_at: DateTime<Utc>,
}

impl Task {
    pub fn new(text: String) -> Task {
        let created_at: DateTime<Utc> = Utc::now();
        Task {
            text: text,
            created_at: created_at,
        }
    }
}

/// Adds the task to the JSON file of tasks
pub fn add_task(journal_path: PathBuf, task: Task) -> Result<()> {
    println!("Adding task [{}] to {:?}", task.text, journal_path);
    let mut file = File::open(journal_path)?;
    let mut buffer = String::new();
    file.read_to_string(&mut buffer)?;
    println!("Got: {}", buffer);
    Ok(())
}

pub fn complete_task(journal_path: PathBuf, task_position: usize) -> Result<()> {
    Ok(())
}

pub fn list_task(journal_path: PathBuf) -> Result<()> {
    Ok(())
}
