use std::error::Error;
use std::thread::Thread;
use std::time::{Duration, SystemTime};
use structopt::StructOpt;
use crate::Cli;

pub fn print_thread_id(thread: &Thread) {
    let args = Cli::from_args();
    if args.show_thread_id {
        println!("Thread id: {:?}", thread.id());
    }
}

pub fn print_error(message: &str, e: &dyn Error) {
    let args = Cli::from_args();
    if args.error {
        eprintln!("{}: {}", message, e);
    }
}

pub fn calculate_duration(start_date: SystemTime, end_date: SystemTime) -> Duration {
    match end_date.duration_since(start_date) {
        Ok(duration) => duration,
        Err(e) => {
            print_error("Error", &e);
            Duration::default()
        }
    }
}