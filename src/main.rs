mod parallel_file_search;
mod utils;

use std::path::PathBuf;
use structopt::StructOpt;

use parallel_file_search::*;

#[derive(Debug, StructOpt)]
pub struct Cli {
    #[structopt(short, long)]
    dir: String,
    #[structopt(short, long)]
    search: String,
    #[structopt(short = "thread-id", long = "show-thread-id", parse(from_flag), takes_value = false)]
    show_thread_id: bool
}

fn main() {
    let args = Cli::from_args();

    for result in parallel_file_search(PathBuf::from(args.dir), args.search) {
        if let Some(path) = result {
            println!("Found match in file: {:?}", path)
        }
    }
}
