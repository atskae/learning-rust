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
        Action::Add {task: new_task} => tasks::add_task(journal_file, Task::new(new_task)),
        Action::Done {position: new_position } => tasks::complete_task(journal_file, new_position),
        Action::List => tasks::list_task(journal_file),
    }.expect("Failed to perform action.");
}
