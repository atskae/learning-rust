use chrono::{serde::ts_seconds, DateTime, Local, Utc};

// Traits that allow de-/serialization
use serde::Deserialize; // get data into the program
use serde::Serialize; // save data outside the program (persistent storage)
use serde_json;

use std::io::{Result, Seek, SeekFrom};
use std::path::{Path, PathBuf};
use std::fs::{File, OpenOptions};
use std::io::Read;
use std::io::Write;
use std::fmt;

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

// Custom print console 
impl fmt::Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Converts UTC to local time
        let created_at = self.created_at.with_timezone(&Local).format("%F %H:%M");
        // Write task and timestamp, x-space padding align
        write!(f, "{:<20} Created at: {}", self.text, created_at)
    }
}

// My horrible attempt 笑 without looking at the solution 
fn old_add_task(journal_path: PathBuf, task: Task) -> Result<()> {
    println!("Adding task [{}] to {:?}", task.text, journal_path);
   
    let mut buffer;
    let mut tasks: Vec::<Task>;
    if !Path::new(&journal_path).exists() {
        println!("Creating a new file.");
        tasks = Vec::<Task>::new();
    } else {
        let mut file = match File::open(&journal_path) {
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
                    tasks = serde_json::from_str(&buffer).unwrap();
                    println!("tasks: {:?}", tasks);
                }
            }
            Err(err) => {
                println!("Got an error... {:?}", err);
                return Err(err);
            }
        };
    }
    // Add task to vector
    tasks.push(task);

    // Save the new vector to a file
    buffer = serde_json::to_string(&tasks).unwrap();
    let mut file = OpenOptions::new()
        .write(true)
        .open(journal_path)
        .unwrap();
    match file.write_all(buffer.as_bytes()){
        Ok(()) => {
            println!("Wrote.");
        }
        Err(err) => {
            println!("Error writing to file; {:?}", err);
            return Err(err);
        }
    };

    println!("Awesome sauce. {}", buffer);
    Ok(())
}

fn get_tasks(mut file: &File) -> Result<Vec<Task>> {
    // Read file and deserialize into Vec<Task>
    let mut tasks: Vec<Task> = match serde_json::from_reader(file) {
        Ok(tasks) => tasks,
        Err(e) if e.is_eof() => Vec::new(), // if empty file
        Err(e) => Err(e)?,
    };
    // Rewind file to beginning
    file.set_len(0); // Remove old contents...
    file.seek(SeekFrom::Start(0))?;

    Ok(tasks)
}

/// Adds the task to the JSON file of tasks
// The solution...
pub fn add_task(journal_path: PathBuf, task: Task) -> Result<()> {
    // Open the file
    // Can apply all modes at the same time
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(journal_path)?;

    let mut tasks: Vec<Task> = get_tasks(&file)?;
    tasks.push(task);
    serde_json::to_writer(file, &tasks);
    
    Ok(())
}

pub fn complete_task(journal_path: PathBuf, task_position: usize) -> Result<()> {
    if !Path::new(&journal_path).exists() {
        return Ok(());
    }

    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .open(journal_path)?;
    let mut tasks = get_tasks(&file)?;
    
    if task_position >= tasks.len() || task_position == 0 {
        return Ok(());
    }

    println!("Removing at position: {}", task_position);
    tasks.remove(task_position-1);
    println!("Contents of tasks: {:?}", tasks);

    serde_json::to_writer(file, &tasks);

    Ok(())
}

pub fn list_task(journal_path: PathBuf) -> Result<()> {
    if !Path::new(&journal_path).exists() {
        return Ok(());
    }

    let mut file = OpenOptions::new()
        .read(true)
        .open(journal_path)?;
    let mut tasks = get_tasks(&file)?;
    if tasks.len() == 0 {
        return Ok(());
    }

    let mut num = 1;
    for task in tasks.iter() {
        //println!("{}: {} (created at {})", num, task.text, task.created_at);
        
        // Since we implemented the Display trait for Task, we can simply print it
        println!("{}: {}", num, task);  
        num += 1;
    }

    Ok(())
}
