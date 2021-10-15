mod cli;
mod tasks;

use std::path::PathBuf;
use structopt::StructOpt;

use cli::Action;
use tasks::Task;

fn main() {
    // Creates a CommandLineArgs struct
    let opt = cli::CommandLineArgs::from_args();
    println!("{:?}", opt);

    let journal_file = match opt.journal_file {
        Some(path) => path,
        None => PathBuf::from("rusty-journal.json"),
    };

    match opt.action {
        Action::Add {task: new_task} => {
            println!("Adding the task: {}", new_task);
            let task = Task::new(new_task);
            tasks::add_task(journal_file, task);
        }
        _ => {
            println!("Not implemented yet!");
        }
    };
}
