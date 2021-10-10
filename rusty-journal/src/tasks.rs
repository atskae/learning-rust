use chrono::{serde::ts_seconds, DateTime, Local, Utc};

// Traits that allow de-/serialization
use serde::Deserialize; // get data into the program
use serde::Serialize; // save data outside the program (persistent storage)

use std::io::Result;
use std::path::PathBuf;


#[derive(Debug, Deserialize, Serialize)]
pub struct Task {
    pub text: String,

    #[serde(with = "ts_seconds")]
    pub created_at: DateTime<Utc>
}

impl Task {
    pub fn new(text: String) -> Task {
        return Task {
            text: text,
            created_at: DateTime.now()
        };
    }
}

pub fn add_task(journal_path: PathBuf, task: Task) -> Result<()> {

}

pub fn complete_task(journal_path: PathBuf, task_position: usize) -> Result<()> {

}

pub fn list_task(journal_path: PathBuf) -> Result<()> {

}
