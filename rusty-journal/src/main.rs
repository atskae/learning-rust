mod cli;
use structopt::StructOpt;

fn main() {
    // Creates a CommandLineArgs struct
    let opt = cli::CommandLineArgs::from_args();
    println!("{:?}", opt);
}
