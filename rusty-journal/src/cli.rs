use std::path::PathBuf; // a "special String" that has path-specific methods
use structopt::StructOpt;


#[derive(Debug, StructOpt)]
pub enum Action {
    /// Add a task to the list
    Add {
        #[structopt()]
        task: String
    },
    
    /// Mark item # `position` as done in the numbered list
    Done {
        #[structopt()]
        position: usize
    },
    
    /// Print task list to the terminal
    List,
}


#[derive(Debug, StructOpt)]
#[structopt(name = "Rusty Journal")]
pub struct CommandLineArgs {
    /// Add a task, mark a task as done, or list all tasks
    #[structopt(subcommand)]
    pub action: Action,
    
    /// Custom filename of the file to save tasks in
    #[structopt(parse(from_os_str), short, long)]
    pub journal_file: Option<PathBuf>,
}
