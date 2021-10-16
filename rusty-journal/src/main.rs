mod cli;
mod tasks;

use anyhow::anyhow;
use std::path::PathBuf;
use structopt::StructOpt;

use cli::Action;
use tasks::Task;

fn find_default_journal_file() -> Option<PathBuf> {
    // home_dir() returns Option<PathBuf>
    home::home_dir().map(|mut path_buf| {
        path_buf.push("rusty-journal.json"); // modify the path
        path_buf // return new path of default file
    })
}

fn main() -> anyhow::Result<()> {
    // Creates a CommandLineArgs struct
    let opt = cli::CommandLineArgs::from_args();
    println!("{:?}", opt);

    let journal_file = opt.journal_file
        .or_else(find_default_journal_file) // takes only a function name
        .ok_or(anyhow!("Failed to find journal file"))?;

    match opt.action {
        Action::Add {task: new_task} => tasks::add_task(journal_file, Task::new(new_task)),
        Action::Done {position: new_position } => tasks::complete_task(journal_file, new_position),
        Action::List => tasks::list_task(journal_file),
    }?;

    Ok(())
}
