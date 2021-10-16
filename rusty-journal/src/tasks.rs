use chrono::{serde::ts_seconds, DateTime, Local, Utc};

// Traits that allow de-/serialization
use serde::Deserialize; // get data into the program
use serde::Serialize; // save data outside the program (persistent storage)
use serde_json;

use std::io::Result;
use std::path::{Path, PathBuf};
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
   
    let mut file;
    let mut buffer;
    let mut tasks;
    if !Path::new(&journal_path).exists() {
        println!("Creating a new file.");
        file = File::create(&journal_path)?;
        tasks = Vec::<Task>::new();
    } else {
        file = match File::open(&journal_path) {
            Ok(f) => f,
            Err(err) => {
                println!("Damn, got an error: {:?}", err);
                return Err(err);
            }
        };
        // Read the file
        buffer = String::new();
        match file.read_to_string(&mut buffer) {
            Ok(bytes_read) => {
                println!("Read {} bytes", bytes_read);
                if bytes_read == 0 {
                    tasks = Vec::<Task>::new();
                } else {
                    // Read string as a vector of Tasks
                    println!("Got: {}", buffer);
                    tasks = serde_json::from_str(&buffer)?;
                    println!("tasks: {:?}", tasks);
                }
            }
            Err(err) => {
                println!("Got an error... {:?}", err);
                return Err(err);
            }
        };
    }
    println!("Awesome sauce.");
    Ok(())
}

pub fn complete_task(journal_path: PathBuf, task_position: usize) -> Result<()> {
    Ok(())
}

pub fn list_task(journal_path: PathBuf) -> Result<()> {
    Ok(())
}
