use std::thread::Thread;
use structopt::StructOpt;
use crate::Cli;

pub fn print_thread_id(thread: &Thread) {
    let args = Cli::from_args();
    if args.show_thread_id {
        println!("Thread id: {:?}", thread.id());
    }
}