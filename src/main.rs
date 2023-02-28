mod parallel_file_search;
mod search_in_file;
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
    show_thread_id: bool,
    #[structopt(short, long, parse(from_flag), takes_value = false)]
    error: bool,
    #[structopt(short, long, default_value = "safe")]
    mode: String,
}

fn main() {
    let args = Cli::from_args();

    for (path, results) in parallel_file_search(PathBuf::from(args.dir), args.search) {
        if results.len() > 0 {
            println!("Found match in file: {:?}", path);
            for result in results {
                println!("Line {} {}\n", result.0, result.1);
            }
        }
    }
}
